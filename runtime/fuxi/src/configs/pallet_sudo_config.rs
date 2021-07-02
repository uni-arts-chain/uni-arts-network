use pallet_sudo::Config;
use crate::*;

impl Config for Runtime {
    type Event = Event;
    type Call = Call;
}