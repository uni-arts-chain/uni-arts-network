use pallet_names::Config;
use crate::*;

impl Config for Runtime {
    type Name = Vec<u8>;
    type Value = Vec<u8>;
    type Currency = Uart;
    type Event = Event;

    fn get_name_fee(op: &pallet_names::Operation<Self>) -> Option<Balance> {
        /* Single-letter names are not allowed (nor the empty name).  Everything
           else is fine.  */
        if op.name.len() < 2 {
            return None
        }

        Some(match op.operation {
            pallet_names::OperationType::Registration => 1000,
            pallet_names::OperationType::Update => 100,
        })
    }

    fn get_expiration(op: &pallet_names::Operation<Self>) -> Option<BlockNumber> {
        /* Short names (up to three characters) will expire after 10 blocks.
           Longer names will stick around forever.  */
        if op.name.len() <= 3 {
            Some(10)
        } else {
            None
        }
    }

    fn deposit_fee(_b: <Self::Currency as Currency<AccountId>>::NegativeImbalance) {
        /* Just burn the name fee by dropping the imbalance.  */
    }
}