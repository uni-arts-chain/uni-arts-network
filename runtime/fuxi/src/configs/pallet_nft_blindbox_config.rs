use pallet_nft_blindbox::Config;
use uniarts_primitives::*;
use crate::*;

impl Config for Runtime {
    type LockModuleId = UniArtsBlindBoxModuleId;
    type NftHandler = Nft;
    type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
    type WeightInfo = ();
}