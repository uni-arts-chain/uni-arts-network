#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::upper_case_acronyms)]

use codec::{Decode, Encode, FullCodec, HasCompact};
use sp_core::H160;
use sp_runtime::{DispatchResult, FixedU128, RuntimeDebug,};
use uniarts_primitives::AccountId;

pub trait NftManager {
    fn transfer_nft(collection_id: u64, item_id: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn transfer_fungible(collection_id: u64, item_id: u64, value: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn transfer_refungible(collection_id: u64, item_id: u64, value: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
}