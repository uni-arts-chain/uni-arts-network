//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
// --- uniarts ---
pub mod chain_spec;
pub mod client;
pub mod service;

pub use service::pangu;
pub use service::fuxi;

pub use chain_spec::pangu::PanguChainSpec;
pub use chain_spec::fuxi::FuxiChainSpec;