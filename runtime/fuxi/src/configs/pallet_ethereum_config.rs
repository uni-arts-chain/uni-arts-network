use pallet_ethereum::Config;
use crate::*;

impl Config for Runtime {
    type Event = Event;
    type StateRoot = pallet_ethereum::IntermediateStateRoot;
}