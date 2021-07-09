//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
// --- uniarts ---
use crate::{client::*, service::*};

// Priority import
pub use sc_service::{
    ChainSpec, Configuration,
};

pub use crate::chain_spec::FuxiChainSpec;
pub use fuxi_runtime;
use uniarts_primitives::{OpaqueBlock as Block};
use uniarts_rpc::fuxi::FullDeps;

use std::{sync::{Arc, Mutex}, time::Duration, collections::{HashMap, BTreeMap}};
use fc_rpc::EthTask;
use fc_rpc_core::types::{FilterPool, PendingTransactions};
use sc_client_api::{ExecutorProvider, RemoteBackend, StateBackendFor, BlockchainEvents};
use sc_service::{error::Error as ServiceError, TaskManager, BasePath, config::KeystoreConfig, PartialComponents};
use fc_consensus::FrontierBlockImport;
use fc_mapping_sync::MappingSyncWorker;
use sp_inherents::InherentDataProviders;
use sc_executor::native_executor_instance;
pub use sc_executor::{NativeExecutor, NativeExecutionDispatch};
use sp_consensus_aura::sr25519::{AuthorityPair as AuraPair};
use sc_finality_grandpa::SharedVoterState;
use sp_api::ConstructRuntimeApi;
use sp_consensus::import_queue::BasicQueue;
use sp_runtime::traits::BlakeTwo256;
use sp_trie::PrefixedMemoryDB;
use futures::StreamExt;

// Our native executor instance.
native_executor_instance!(
	pub FuxiExecutor,
	fuxi_runtime::api::dispatch,
	fuxi_runtime::native_version,
	frame_benchmarking::benchmarking::HostFunctions,
);

pub fn frontier_database_dir(config: &Configuration) -> std::path::PathBuf {
    let config_dir = config.base_path.as_ref()
        .map(|base_path| base_path.config_dir(config.chain_spec.id()))
        .unwrap_or_else(|| {
            BasePath::from_project("", "", "fuxi")
                .config_dir(config.chain_spec.id())
        });
    config_dir.join("frontier").join("db")
}

pub fn open_frontier_backend(config: &Configuration) -> Result<Arc<fc_db::Backend<Block>>, String> {
    Ok(Arc::new(fc_db::Backend::<Block>::new(&fc_db::DatabaseSettings {
        source: fc_db::DatabaseSettingsSrc::RocksDb {
            path: frontier_database_dir(&config),
            cache_size: 0,
        }
    })?))
}

pub fn new_partial<RuntimeApi, Executor>(config: &mut Configuration) -> Result<sc_service::PartialComponents<
    FullClient<RuntimeApi, Executor>,
    FullBackend,
    FullSelectChain,
    sp_consensus::DefaultImportQueue<Block, FullClient<RuntimeApi, Executor>>,
    sc_transaction_pool::FullPool<Block, FullClient<RuntimeApi, Executor>>,
    (
        (sc_consensus_aura::AuraBlockImport<
            Block,
            FullClient<RuntimeApi, Executor>,
            FrontierBlockImport<
                Block,
                sc_finality_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient<RuntimeApi, Executor>, FullSelectChain>,
                FullClient<RuntimeApi, Executor>
            >,
            AuraPair
        >,
         sc_finality_grandpa::LinkHalf<Block, FullClient<RuntimeApi, Executor>, FullSelectChain>),
        PendingTransactions, Option<FilterPool>, Arc<fc_db::Backend<Block>>
    )
>, ServiceError>
    where
        Executor: 'static + NativeExecutionDispatch,
        RuntimeApi: 'static + Send + Sync + ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
        RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = StateBackendFor<FullBackend, Block>>,
{
    set_prometheus_registry(config)?;

    if config.keystore_remote.is_some() {
        return Err(ServiceError::Other(
            format!("Remote Keystores are not supported.")))
    }

    let inherent_data_providers = sp_inherents::InherentDataProviders::new();

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, Executor>(&config)?;
    let client = Arc::new(client);

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority().into(),
        config.prometheus_registry(),
        task_manager.spawn_handle(),
        client.clone(),
    );

    let pending_transactions: PendingTransactions
        = Some(Arc::new(Mutex::new(HashMap::new())));

    let filter_pool: Option<FilterPool>
        = Some(Arc::new(Mutex::new(BTreeMap::new())));

    let frontier_backend = open_frontier_backend(config)?;

    // inherent_data_providers
    //     .register_provider(pallet_dynamic_fee::InherentDataProvider(U256::from(cli.run.target_gas_price)))
    //     .map_err(Into::into)
    //     .map_err(sp_consensus::Error::InherentData)?;

    let (grandpa_block_import, grandpa_link) = sc_finality_grandpa::block_import(
        client.clone(), &(client.clone() as Arc<_>), select_chain.clone(),
    )?;

    let frontier_block_import = FrontierBlockImport::new(
        grandpa_block_import.clone(),
        client.clone(),
        frontier_backend.clone(),
    );

    let aura_block_import = sc_consensus_aura::AuraBlockImport::<_, _, _, AuraPair>::new(
        frontier_block_import.clone(), client.clone(),
    );

    let import_queue = sc_consensus_aura::import_queue::<_, _, _, AuraPair, _, _>(
        sc_consensus_aura::slot_duration(&*client)?,
        aura_block_import.clone(),
        Some(Box::new(grandpa_block_import.clone())),
        client.clone(),
        inherent_data_providers.clone(),
        &task_manager.spawn_handle(),
        config.prometheus_registry(),
        sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone()),
    )?;

    Ok(sc_service::PartialComponents {
        client, backend, task_manager, import_queue, keystore_container, select_chain, transaction_pool,
        inherent_data_providers,
        other: (
            (aura_block_import, grandpa_link),
            pending_transactions,
            filter_pool,
            frontier_backend,
        )
    })
}

/// Builds a new service for a full client.
pub fn new_full<RuntimeApi, Executor>(mut config: Configuration) -> Result<(TaskManager, Arc<FullClient<RuntimeApi, Executor>>), ServiceError>
    where
        Executor: 'static + NativeExecutionDispatch,
        RuntimeApi: 'static + Send + Sync + ConstructRuntimeApi<Block, FullClient<RuntimeApi, Executor>>,
        RuntimeApi::RuntimeApi: RuntimeApiCollection<StateBackend = StateBackendFor<FullBackend, Block>>,
{
    // let enable_dev_signer = cli.run.enable_dev_signer;
    let enable_dev_signer = false;

    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        mut keystore_container,
        select_chain,
        transaction_pool,
        inherent_data_providers,
        other: ((block_import, grandpa_link),
            pending_transactions,
            filter_pool,
            frontier_backend),
    } = new_partial(&mut config)?;

    if let Some(url) = &config.keystore_remote {
        match remote_keystore(url) {
            Ok(k) => keystore_container.set_remote_keystore(k),
            Err(e) => {
                return Err(ServiceError::Other(
                    format!("Error hooking up remote keystore for {}: {}", url, e)))
            }
        };
    }

    config.network.extra_sets.push(sc_finality_grandpa::grandpa_peers_set_config());

    // let finality_proof_provider =
    //     GrandpaFinalityProofProvider::new_for_service(backend.clone(), client.clone());

    let (network, network_status_sinks, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            on_demand: None,
            block_announce_validator_builder: None,
        })?;

    // Channel for the rpc handler to communicate with the authorship task.
    let (command_sink, _commands_stream) = futures::channel::mpsc::channel(1000);

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config, backend.clone(), task_manager.spawn_handle(), client.clone(), network.clone(),
        );
    }

    let role = config.role.clone();
    let force_authoring = config.force_authoring;
    let backoff_authoring_blocks: Option<()> = None;
    let name = config.network.node_name.clone();
    let enable_grandpa = !config.disable_grandpa;
    let prometheus_registry = config.prometheus_registry().cloned();
    let is_authority = role.is_authority();
    let subscription_task_executor = sc_rpc::SubscriptionTaskExecutor::new(task_manager.spawn_handle());

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();
        let network = network.clone();
        let pending = pending_transactions.clone();
        let filter_pool = filter_pool.clone();
        let frontier_backend = frontier_backend.clone();
        let max_past_logs = 10000u32;

        Box::new(move |deny_unsafe, _| {
            let deps = FullDeps {
                client: client.clone(),
                pool: pool.clone(),
                deny_unsafe,
                is_authority,
                enable_dev_signer,
                network: network.clone(),
                pending_transactions: pending.clone(),
                filter_pool: filter_pool.clone(),
                backend: frontier_backend.clone(),
                max_past_logs,
                command_sink: Some(command_sink.clone()),
            };

            uniarts_rpc::fuxi::create_full(deps, subscription_task_executor.clone())
        })
    };

    task_manager.spawn_essential_handle().spawn(
        "frontier-mapping-sync-worker",
        MappingSyncWorker::new(
            client.import_notification_stream(),
            Duration::new(6, 0),
            client.clone(),
            backend.clone(),
            frontier_backend.clone(),
        ).for_each(|()| futures::future::ready(()))
    );

    let (_rpc_handlers, telemetry_connection_notifier) = sc_service::spawn_tasks(
        sc_service::SpawnTasksParams {
            network: network.clone(),
            client: client.clone(),
            keystore: keystore_container.sync_keystore(),
            task_manager: &mut task_manager,
            transaction_pool: transaction_pool.clone(),
            rpc_extensions_builder,
            on_demand: None,
            remote_blockchain: None,
            backend,
            network_status_sinks,
            system_rpc_tx,
            config,
        },
    )?;

    // Spawn Frontier EthFilterApi maintenance task.
    if let Some(filter_pool) = filter_pool {
        // Each filter is allowed to stay in the pool for 100 blocks.
        const FILTER_RETAIN_THRESHOLD: u64 = 100;
        task_manager.spawn_essential_handle().spawn(
            "frontier-filter-pool",
            EthTask::filter_pool_task(
                Arc::clone(&client),
                filter_pool,
                FILTER_RETAIN_THRESHOLD,
            )
        );
    }

    // Spawn Frontier pending transactions maintenance task (as essential, otherwise we leak).
    if let Some(pending_transactions) = pending_transactions {
        const TRANSACTION_RETAIN_THRESHOLD: u64 = 5;
        task_manager.spawn_essential_handle().spawn(
            "frontier-pending-transactions",
            EthTask::pending_transaction_task(
                Arc::clone(&client),
                pending_transactions,
                TRANSACTION_RETAIN_THRESHOLD,
            )
        );
    }

    if role.is_authority() {
        let proposer = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool,
            prometheus_registry.as_ref(),
        );

        let can_author_with =
            sp_consensus::CanAuthorWithNativeVersion::new(client.executor().clone());

        let aura = sc_consensus_aura::start_aura::<_, _, _, _, _, AuraPair, _, _, _, _>(
            sc_consensus_aura::slot_duration(&*client)?,
            client.clone(),
            select_chain,
            block_import,
            proposer,
            network.clone(),
            inherent_data_providers.clone(),
            force_authoring,
            backoff_authoring_blocks,
            keystore_container.sync_keystore(),
            can_author_with,
        )?;

        // the AURA authoring task is considered essential, i.e. if it
        // fails we take down the service with it.
        task_manager.spawn_essential_handle().spawn_blocking("aura", aura);
    }

    // if the node isn't actively participating in consensus then it doesn't
    // need a keystore, regardless of which protocol we use below.
    let keystore = if role.is_authority() {
        Some(keystore_container.sync_keystore())
    } else {
        None
    };

    let grandpa_config = sc_finality_grandpa::Config {
        // FIXME #1578 make this available through chainspec
        gossip_duration: Duration::from_millis(333),
        justification_period: 512,
        name: Some(name),
        observer_enabled: false,
        keystore,
        is_authority: role.is_network_authority(),
    };

    if enable_grandpa {
        // start the full GRANDPA voter
        // NOTE: non-authorities could run the GRANDPA observer protocol, but at
        // this point the full voter should provide better guarantees of block
        // and vote data availability than the observer. The observer has not
        // been tested extensively yet and having most nodes in a chain run it
        // could lead to finality stalls.
        let grandpa_config = sc_finality_grandpa::GrandpaParams {
            config: grandpa_config,
            link: grandpa_link,
            network,
            telemetry_on_connect: telemetry_connection_notifier.map(|x| x.on_connect_stream()),
            voting_rule: sc_finality_grandpa::VotingRulesBuilder::default().build(),
            prometheus_registry,
            shared_voter_state: SharedVoterState::empty(),
        };

        // the GRANDPA voter task is considered infallible, i.e.
        // if it fails we take down the service with it.
        task_manager.spawn_essential_handle().spawn_blocking(
            "grandpa-voter",
            sc_finality_grandpa::run_grandpa_voter(grandpa_config)?
        );
    }

    network_starter.start_network();
    Ok((task_manager, client))
}

/// Builds a new service for a light client.
pub fn new_light<RuntimeApi, Executor>(mut config: Configuration) -> Result<TaskManager, ServiceError>
    where
        Executor: 'static + NativeExecutionDispatch,
        RuntimeApi:
        'static + Send + Sync + ConstructRuntimeApi<Block, LightClient<RuntimeApi, Executor>>,
        <RuntimeApi as ConstructRuntimeApi<Block, LightClient<RuntimeApi, Executor>>>::RuntimeApi:
        RuntimeApiCollection<StateBackend = StateBackendFor<LightBackend, Block>>,
{
    set_prometheus_registry(&mut config)?;

    let (client, backend, keystore_container, mut task_manager, on_demand) =
        sc_service::new_light_parts::<Block, RuntimeApi, Executor>(&config)?;

    config.network.extra_sets.push(sc_finality_grandpa::grandpa_peers_set_config());

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_light(
        config.transaction_pool.clone(),
        config.prometheus_registry(),
        task_manager.spawn_handle(),
        client.clone(),
        on_demand.clone(),
    ));

    let (grandpa_block_import, _) = sc_finality_grandpa::block_import(
        client.clone(),
        &(client.clone() as Arc<_>),
        select_chain.clone(),
    )?;

    let aura_block_import = sc_consensus_aura::AuraBlockImport::<_, _, _, AuraPair>::new(
        grandpa_block_import.clone(),
        client.clone(),
    );

    let import_queue = sc_consensus_aura::import_queue::<_, _, _, AuraPair, _, _>(
        sc_consensus_aura::slot_duration(&*client)?,
        aura_block_import,
        Some(Box::new(grandpa_block_import)),
        client.clone(),
        InherentDataProviders::new(),
        &task_manager.spawn_handle(),
        config.prometheus_registry(),
        sp_consensus::NeverCanAuthor,
    )?;

    let (network, network_status_sinks, system_rpc_tx, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            on_demand: Some(on_demand.clone()),
            block_announce_validator_builder: None,
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config, backend.clone(), task_manager.spawn_handle(), client.clone(), network.clone(),
        );
    }

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        remote_blockchain: Some(backend.remote_blockchain()),
        transaction_pool,
        task_manager: &mut task_manager,
        on_demand: Some(on_demand),
        rpc_extensions_builder: Box::new(|_, _| ()),
        config,
        client,
        keystore: keystore_container.sync_keystore(),
        backend,
        network,
        network_status_sinks,
        system_rpc_tx,
    })?;

    network_starter.start_network();

    Ok(task_manager)
}

/// Builds a new object suitable for chain operations.
#[cfg(feature = "full-node")]
pub fn new_chain_ops<Runtime, Dispatch>(
    config: &mut Configuration,
) -> Result<
    (
        Arc<Client>,
        Arc<FullBackend>,
        BasicQueue<Block, PrefixedMemoryDB<BlakeTwo256>>,
        TaskManager,
    ),
    ServiceError,
>
    where
        Dispatch: 'static + NativeExecutionDispatch,
        Runtime: 'static + Send + Sync + ConstructRuntimeApi<Block, FullClient<Runtime, Dispatch>>,
        Runtime::RuntimeApi: RuntimeApiCollection<StateBackend = StateBackendFor<FullBackend, Block>>,
{
    config.keystore = KeystoreConfig::InMemory;

    let PartialComponents {
        client,
        backend,
        import_queue,
        task_manager,
        ..
    } = new_partial::<fuxi_runtime::RuntimeApi, FuxiExecutor>(config)?;

    Ok((Arc::new(Client::Fuxi(client)), backend, import_queue, task_manager))
}

/// Create a new Uniarts service for a full node.
#[cfg(feature = "full-node")]
pub fn fuxi_new_full(
    config: Configuration,
) -> Result<
    (
        TaskManager,
        Arc<Client>,
    ),
    ServiceError,
> {
    let (components, client) = new_full::<fuxi_runtime::RuntimeApi, FuxiExecutor>(config)?;

    Ok((components, Arc::new(Client::Fuxi(client))))
}

/// Create a new Uniarts service for a light client.
pub fn fuxi_new_light(config: Configuration) -> Result<TaskManager, ServiceError> {
    new_light::<fuxi_runtime::RuntimeApi, FuxiExecutor>(config)
}
