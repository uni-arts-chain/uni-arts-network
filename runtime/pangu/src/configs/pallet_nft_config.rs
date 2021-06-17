use pallet_nft::Config;
use uniarts_primitives::*;
use crate::*;

/// Used for the module nft in `./nft.rs`
impl Config for Runtime {
    type ModuleId = UniArtsNftModuleId;
    type Currency = Uart;
    type Event = Event;
    type WeightInfo = ();
}