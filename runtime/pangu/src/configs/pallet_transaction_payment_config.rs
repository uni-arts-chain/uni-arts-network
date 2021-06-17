use pallet_transaction_payment::Config;
use sp_runtime::Perquintill;

use uniarts_primitives::*;
pub use crate::constants::currency::*;
use crate::*;

pub struct DealWithFees;

impl OnUnbalanced<NegativeImbalance<Runtime>> for DealWithFees {
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<Runtime>>) {
        if let Some(fees) = fees_then_tips.next() {
            // for fees, 90% to treasury, 10% to author
            let mut split = fees.ration(90, 10);
            if let Some(tips) = fees_then_tips.next() {
                // for tips, if any, 90% to treasury, 10% to author (though this can be anything)
                tips.ration_merge_into(90, 10, &mut split);
            }
            Treasury::on_unbalanced(split.0);
            Author::on_unbalanced(split.1);
        }
    }
}

frame_support::parameter_types! {
	pub const TransactionByteFee: Balance = 1 * MICRO;
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
}

impl Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Uart, DealWithFees>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate =
    TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
}