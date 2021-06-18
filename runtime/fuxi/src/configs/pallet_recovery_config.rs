use pallet_recovery::Config;
use uniarts_primitives::*;
use crate::*;

frame_support::parameter_types! {
	pub const ConfigDepositBase: Balance = 10 * MILLI;
	pub const FriendDepositFactor: Balance = MILLI;
	pub const MaxFriends: u16 = 9;
	pub const RecoveryDeposit: Balance = 10 * MILLI;
}

impl Config for Runtime {
    type Event = Event;
    type Call = Call;
    type Currency = Uart;
    type ConfigDepositBase = ConfigDepositBase;
    type FriendDepositFactor = FriendDepositFactor;
    type MaxFriends = MaxFriends;
    type RecoveryDeposit = RecoveryDeposit;
}