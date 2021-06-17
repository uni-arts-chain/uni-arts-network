use pallet_nft_multi::Config;
use uniarts_primitives::*;
use crate::*;

/// Used for the module nft-multi in `./nft-multi.rs`
impl Config for Runtime {
    type ModuleId = UniArtsNftModuleId;
    type MultiCurrency = Currencies;
    type Event = Event;
    type WeightInfo = ();
}