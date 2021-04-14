#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::upper_case_acronyms)]

use sp_runtime::{DispatchResult};

pub trait NftManager<AccountId> {
    fn transfer_nft(collection_id: u64, item_id: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn transfer_fungible(collection_id: u64, item_id: u64, value: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn transfer_refungible(collection_id: u64, item_id: u64, value: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
}