//! Weights for pallet_utility
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0
//! DATE: 2020-09-28, STEPS: [50], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
    fn batch(c: u32) -> Weight {
        (18_450_000 as Weight).saturating_add((1_730_000 as Weight).saturating_mul(c as Weight))
    }
    fn as_derivative() -> Weight {
        (5_548_000 as Weight)
    }
    fn batch_all(c: u32) -> Weight {
        (19_735_000 as Weight).saturating_add((1_990_000 as Weight).saturating_mul(c as Weight))
    }
}