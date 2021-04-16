#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::upper_case_acronyms)]

use sp_runtime::{DispatchResult, FixedU128};

pub type Price = FixedU128;

pub trait NftManager<AccountId> {
    fn transfer_nft(collection_id: u64, item_id: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn transfer_fungible(collection_id: u64, item_id: u64, value: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
    fn transfer_refungible(collection_id: u64, item_id: u64, value: u64, sender: AccountId, new_owner: AccountId) -> DispatchResult;
}

pub trait PriceProvider<CurrencyId> {
    fn get_relative_price(base: CurrencyId, quote: CurrencyId) -> Option<Price>;
    fn get_price(currency_id: CurrencyId) -> Option<Price>;
    fn lock_price(currency_id: CurrencyId);
    fn unlock_price(currency_id: CurrencyId);
}