use pallet_ethereum::Config;
use crate::*;
use sp_std::marker::PhantomData;
use sp_core::{U256, H160};
use sp_core::crypto::Public;

pub struct EthereumFindAuthor<F>(PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for EthereumFindAuthor<F>
{
    fn find_author<'a, I>(digests: I) -> Option<H160> where
        I: 'a + IntoIterator<Item=(ConsensusEngineId, &'a [u8])>
    {
        if let Some(author_index) = F::find_author(digests) {
            let authority_id = Aura::authorities()[author_index as usize].clone();
            return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]));
        }
        None
    }
}

frame_support::parameter_types! {
	pub BlockGasLimit: U256 = U256::from(u32::max_value());
}

impl Config for Runtime {
    type Event = Event;
    type FindAuthor = EthereumFindAuthor<Aura>;
    type StateRoot = pallet_ethereum::IntermediateStateRoot;
    type BlockGasLimit = BlockGasLimit;
}