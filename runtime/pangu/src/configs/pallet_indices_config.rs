use pallet_indices::Config;
use uniarts_primitives::*;
use crate::{weights::pallet_indices::WeightInfo, *};

frame_support::parameter_types! {
	pub const IndexDeposit: Balance = 1 * UART;
}
impl Config for Runtime {
    type AccountIndex = AccountIndex;
    type Currency = Uart;
    type Deposit = IndexDeposit;
    type Event = Event;
    type WeightInfo =WeightInfo<Runtime>;
}