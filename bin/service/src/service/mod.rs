pub mod pangu;
pub mod fuxi;

// --- std ---
use std::sync::Arc;
// --- substrate ---
use sc_keystore::LocalKeystore;
use sc_service::{config::PrometheusConfig, ChainSpec, Configuration, Error as ServiceError};
use sp_runtime::traits::BlakeTwo256;
use substrate_prometheus_endpoint::Registry;

use uniarts_primitives::{OpaqueBlock as Block};

type FullClient<RuntimeApi, Executor> = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
// type FullGrandpaBlockImport<RuntimeApi, Executor> = sc_finality_grandpa::GrandpaBlockImport<
//     FullBackend,
//     Block,
//     FullClient<RuntimeApi, Executor>,
//     FullSelectChain,
// >;
type LightBackend = sc_service::TLightBackendWithHash<Block, BlakeTwo256>;
type LightClient<RuntimeApi, Executor> =
sc_service::TLightClientWithBackend<Block, RuntimeApi, Executor, LightBackend>;

/// Can be called for a `Configuration` to check if it is a configuration for the `pangu` chain.
pub trait IdentifyVariant {
    /// Returns if this is a configuration for the `Pangu` chain.
    fn is_pangu_network(&self) -> bool;

    /// Returns if this is a configuration for the `Fuxi` chain.
    fn is_fuxi_network(&self) -> bool;
}
impl IdentifyVariant for Box<dyn ChainSpec> {
    fn is_pangu_network(&self) -> bool {
        self.id().starts_with("pangu") || self.id().starts_with("uart")
    }

    fn is_fuxi_network(&self) -> bool {
        self.id().starts_with("fuxi")
    }
}

// If we're using prometheus, use a registry with a prefix of `uniarts`.
fn set_prometheus_registry(config: &mut Configuration) -> Result<(), ServiceError> {
    if let Some(PrometheusConfig { registry, .. }) = config.prometheus_config.as_mut() {
        *registry = Registry::new_custom(Some("uniarts".into()), None)?;
    }

    Ok(())
}

fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
    // FIXME: here would the concrete keystore be built,
    //        must return a concrete type (NOT `LocalKeystore`) that
    //        implements `CryptoStore` and `SyncCryptoStore`
    Err("Remote Keystore not supported.")
}
