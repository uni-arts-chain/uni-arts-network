#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::upper_case_acronyms)]

use sp_runtime::{DispatchResult, FixedU128};
use uniarts_primitives::CurrencyId;

pub type Price = FixedU128;

pub trait NftManager<AccountId, BlockNumber> {
    fn transfer_nft(collection_id: u64, item_id: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn transfer_fungible(collection_id: u64, item_id: u64, value: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn transfer_refungible(collection_id: u64, item_id: u64, value: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn is_item_owner(subject: AccountId, collection_id: u64, item_id: u64) -> bool;
    fn charge_royalty(buyer: AccountId, collection_id: u64, item_id: u64, currency_id: CurrencyId, order_price: u64, now: BlockNumber) -> DispatchResult;
}

pub trait PriceProvider<CurrencyId> {
    fn get_relative_price(base: CurrencyId, quote: CurrencyId) -> Option<Price>;
    fn get_price(currency_id: CurrencyId) -> Option<Price>;
    fn lock_price(currency_id: CurrencyId);
    fn unlock_price(currency_id: CurrencyId);
}