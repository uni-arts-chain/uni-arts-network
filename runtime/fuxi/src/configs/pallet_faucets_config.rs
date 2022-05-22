use pallet_faucets::Config;
use uniarts_primitives::*;
use crate::*;

impl Config for Runtime {
	type Event = Event;
	type Currency = Uart;
}