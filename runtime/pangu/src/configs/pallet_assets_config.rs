use pallet_assets::Config;
use uniarts_primitives::*;
use crate::*;
use crate::constants::currency::*;

frame_support::parameter_types! {
	pub const AssetDepositBase: Balance = 100 * DOLLARS;
	pub const AssetDepositPerZombie: Balance = 1 * DOLLARS;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10 * DOLLARS;
	pub const MetadataDepositPerByte: Balance = 1 * DOLLARS;
}

impl Config for Runtime {
    type Event = Event;
    type Balance = u64;
    type AssetId = u32;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDepositBase = AssetDepositBase;
    type AssetDepositPerZombie = AssetDepositPerZombie;
    type StringLimit = StringLimit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
}