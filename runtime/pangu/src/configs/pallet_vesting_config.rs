use pallet_vesting::Config;
use sp_runtime::traits::ConvertInto;
use crate::*;

frame_support::parameter_types! {
	pub const MinVestedTransfer: Balance = 100 * DOLLARS;
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Uart;
    type BlockNumberToBalance = ConvertInto;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = ();
}