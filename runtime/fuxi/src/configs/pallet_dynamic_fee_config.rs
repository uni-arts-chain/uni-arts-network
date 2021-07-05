use pallet_dynamic_fee::Config;
use crate::*;
use sp_core::U256;

frame_support::parameter_types! {
	pub BoundDivision: U256 = U256::from(1024);
}

impl Config for Runtime {
    type MinGasPriceBoundDivisor = BoundDivision;
}