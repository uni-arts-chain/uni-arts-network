//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

impl crate::WeightInfo for () {
    fn create_blind_box() -> Weight {
        (66_234_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }

    fn blind_box_add_card_group() -> Weight {
        (66_234_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }

    fn blind_box_remove_card_group() -> Weight {
        (66_234_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }

    fn buy_blind_box() -> Weight {
        (66_234_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }

    fn close_blind_box() -> Weight {
        (66_234_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }

    fn open_blind_box() -> Weight {
        (66_234_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }

    fn cancel_blind_box() -> Weight {
            (66_234_000 as Weight)
                .saturating_add(DbWeight::get().reads(5 as Weight))
                .saturating_add(DbWeight::get().writes(3 as Weight))
    }
}