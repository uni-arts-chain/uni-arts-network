use sp_runtime::Perbill;
use uniarts_primitives::*;
use pallet_validator_set::Config;
use crate::constants::currency::*;
use crate::*;

frame_support::parameter_types! {
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
	pub const ValidatorMortgageLimit: Balance = 10_000 * UART;
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Uart;
    type ValidatorMortgageLimit = ValidatorMortgageLimit;
}