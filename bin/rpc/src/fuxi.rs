//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use uniarts_primitives::{OpaqueBlock as Block, AccountId, Balance, Index, BlockNumber, Nonce, Hash};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::{Error as BlockChainError, HeaderMetadata, HeaderBackend};
use sp_block_builder::BlockBuilder;
pub use sc_rpc_api::DenyUnsafe;
use sp_transaction_pool::TransactionPool;

use sc_client_api::{
    backend::{StorageProvider, Backend, StateBackend, AuxStore},
    client::BlockchainEvents
};
use sc_transaction_graph::{ChainApi, Pool};
use sc_rpc::SubscriptionTaskExecutor;
use sp_runtime::traits::BlakeTwo256;
use sp_block_builder::BlockBuilder;
use sc_network::NetworkService;
use jsonrpc_pubsub::manager::SubscriptionManager;

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpc_core::IoHandler<sc_rpc::Metadata>;

/// Full client dependencies.
pub struct FullDeps<C, P, A: ChainApi> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// The Node authority flag
    pub is_authority: bool,
    /// Whether to enable dev signer
    pub enable_dev_signer: bool,
    /// Network service
    pub network: Arc<NetworkService<Block, Hash>>,
    /// Ethereum pending transactions.
    pub pending_transactions: PendingTransactions,
    /// EthFilterApi pool.
    pub filter_pool: Option<FilterPool>,
    /// Manual seal command sink
    pub command_sink: Option<futures::channel::mpsc::Sender<sc_consensus_manual_seal::rpc::EngineCommand<Hash>>>,
}

/// Light client extra dependencies.
pub struct LightDeps<C, F, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Remote access to the blockchain (async).
    pub remote_blockchain: Arc<dyn sc_client_api::light::RemoteBlockchain<Block>>,
    /// Fetcher instance.
    pub fetcher: Arc<F>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, BE, A>(
    deps: FullDeps<C, P, A>,
    subscription_task_executor: SubscriptionTaskExecutor
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata> where
    BE: Backend<Block> + 'static,
    BE::State: StateBackend<BlakeTwo256>,
    C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
    C: BlockchainEvents<Block>,
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error=BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: pallet_staking_rpc::StakingRuntimeApi<Block, AccountId, Balance>,
    C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
    C::Api: BlockBuilder<Block>,
    C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
    P: TransactionPool<Block=Block> + 'static,
    A: ChainApi<Block = Block> + 'static,
{
    use substrate_frame_rpc_system::{FullSystem, SystemApi};
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
    use pallet_staking_rpc::{Staking, StakingApi};
    use pallet_contracts_rpc::{Contracts, ContractsApi};
    use fc_rpc::{
        EthApi, EthApiServer, EthFilterApi, EthFilterApiServer, NetApi, NetApiServer,
        EthPubSubApi, EthPubSubApiServer, Web3Api, Web3ApiServer, EthDevSigner, EthSigner,
        HexEncodedIdProvider,
    };

    let mut io = jsonrpc_core::IoHandler::default();
    let FullDeps {
        client,
        pool,
        deny_unsafe,
        is_authority,
        network,
        pending_transactions,
        filter_pool,
        command_sink,
        enable_dev_signer,
    } = deps;

    io.extend_with(
        SystemApi::to_delegate(FullSystem::new(client.clone(), pool, deny_unsafe))
    );

    io.extend_with(
        TransactionPaymentApi::to_delegate(TransactionPayment::new(client.clone()))
    );

    let mut signers = Vec::new();
    if enable_dev_signer {
        signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
    }
    io.extend_with(
        EthApiServer::to_delegate(EthApi::new(
            client.clone(),
            pool.clone(),
            graph,
            fuxi_runtime::TransactionConverter,
            network.clone(),
            pending_transactions.clone(),
            signers,
            is_authority,
        ))
    );

    if let Some(filter_pool) = filter_pool {
        io.extend_with(
            EthFilterApiServer::to_delegate(EthFilterApi::new(
                client.clone(),
                filter_pool.clone(),
                500 as usize, // max stored filters
            ))
        );
    }

    io.extend_with(
        NetApiServer::to_delegate(NetApi::new(
            client.clone(),
            network.clone(),
        ))
    );

    io.extend_with(
        Web3ApiServer::to_delegate(Web3Api::new(
            client.clone(),
        ))
    );

    io.extend_with(
        EthPubSubApiServer::to_delegate(EthPubSubApi::new(
            pool.clone(),
            client.clone(),
            network.clone(),
            SubscriptionManager::<HexEncodedIdProvider>::with_id_provider(
                HexEncodedIdProvider::default(),
                Arc::new(subscription_task_executor)
            ),
        ))
    );

    io.extend_with(ContractsApi::to_delegate(Contracts::new(client.clone())));

    io.extend_with(
        StakingApi::to_delegate(Staking::new(client.clone()))
    );

    // Extend this RPC with a custom API by using the following syntax.
    // `YourRpcStruct` should have a reference to a client, which is needed
    // to call into the runtime.
    // `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`
    io
}

/// Instantiate all RPC extensions for light node.
pub fn create_light<C, P, M, F>(deps: LightDeps<C, F, P>) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
    where
        C: Send + Sync + 'static,
        C: sp_blockchain::HeaderBackend<Block>,
        F: sc_client_api::light::Fetcher<Block> + 'static,
        P: sp_transaction_pool::TransactionPool + 'static,
        M: jsonrpc_core::Metadata + Default,
{
    // --- substrate ---
    use substrate_frame_rpc_system::{LightSystem, SystemApi};

    let LightDeps {
        client,
        pool,
        remote_blockchain,
        fetcher,
    } = deps;
    let mut io = jsonrpc_core::IoHandler::default();

    io.extend_with(SystemApi::<Hash, AccountId, Nonce>::to_delegate(
        LightSystem::new(client, remote_blockchain, fetcher, pool),
    ));

    io
}