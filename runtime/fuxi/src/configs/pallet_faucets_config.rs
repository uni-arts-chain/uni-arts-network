use pallet_faucets::Config;
use crate::*;

impl Config for Runtime {
	type Event = Event;
	type Currency = Uart;
}