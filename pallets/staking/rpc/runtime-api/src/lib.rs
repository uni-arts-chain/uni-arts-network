#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait StakingApi<AccountId, Balance> where
		AccountId: codec::Codec,
		Balance: codec::Codec
	{
		fn staking_module_account_id() -> AccountId;
		fn pool_account_id(id: u32) -> AccountId;
		fn pending_rewards(pool_id: u32, account_id: AccountId) -> Balance;
	}
}