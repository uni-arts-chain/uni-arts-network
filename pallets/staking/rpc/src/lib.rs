use std::sync::Arc;
use codec::{Codec, HasCompact};
use jsonrpc_derive::rpc;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::{Block as BlockT, MaybeDisplay, MaybeFromStr}};
use sp_api::ProvideRuntimeApi;
pub use pallet_staking_rpc_runtime_api::StakingApi as StakingRuntimeApi;

#[rpc]
pub trait StakingApi<AccountId, Balance> {
	#[rpc(name = "staking_pendingRewards")]
	fn pending_rewards(
		&self, 
		account_id: AccountId
	) -> Result<Balance>;

	#[rpc(name = "staking_poolAccountId")]
	fn pool_account_id(&self) -> Result<AccountId>;
}

pub struct Staking<C, P> {
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Staking<C, P> {
	pub fn new(client: Arc<C>) -> Self {
		Staking { client, _marker: Default::default() }
	}
}


impl<C, Block, AccountId, Balance> StakingApi<AccountId, Balance>
	for Staking<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
	C::Api: StakingRuntimeApi<Block, AccountId, Balance>,
	Balance: Codec + MaybeDisplay + MaybeFromStr + HasCompact, 
	AccountId: Codec
{

	fn pending_rewards(&self, account_id: AccountId) -> Result<Balance> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);
		api.pending_rewards(&at, 0, account_id).map_err(|e| RpcError {
			code: ErrorCode::InternalError,
			message: "Unable to query pending rewards".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}

	fn pool_account_id(&self) -> Result<AccountId> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(self.client.info().best_hash);
		api.staking_module_account_id(&at).map_err(|e| RpcError {
			code: ErrorCode::InternalError,
			message: "Unable to query pool account_id".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}