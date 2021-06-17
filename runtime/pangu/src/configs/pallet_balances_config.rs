use pallet_balances::Config;
use uniarts_primitives::*;
use crate::*;
use crate::weights::pallet_balances::WeightInfo;

frame_support::parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
}

// type UartInstance = pallet_balances::Instance0;
pub type UinkInstance = pallet_balances::Instance1;

impl Config for Runtime {
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = MaxLocks;
    type WeightInfo = WeightInfo<Runtime>;
}

type UinkAccountStore = StorageMapShim<
    pallet_balances::Account<Runtime, UinkInstance>,
    frame_system::Provider<Runtime>,
    Balance,
    pallet_balances::AccountData<Balance>
>;

impl Config<UinkInstance> for Runtime {
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = UinkAccountStore;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}