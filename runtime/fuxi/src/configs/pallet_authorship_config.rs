use pallet_authorship::Config;
use crate::*;

pub struct AuraAccountAdapter;

impl FindAuthor<AccountId> for AuraAccountAdapter {
    fn find_author<'a, I>(digests: I) -> Option<AccountId>
        where I: 'a + IntoIterator<Item=(ConsensusEngineId, &'a [u8])>
    {
        if let Some(index) = pallet_aura::Module::<Runtime>::find_author(digests) {
            let validator = pallet_session::Module::<Runtime>::validators()[index as usize].clone();
            Some(validator)
        }
        else {
            None
        }
    }
}

frame_support::parameter_types! {
	pub const UncleGenerations: BlockNumber = 0;
}

impl Config for Runtime {
    type FindAuthor = AuraAccountAdapter;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = ();
}