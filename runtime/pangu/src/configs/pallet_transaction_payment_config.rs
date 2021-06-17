use pallet_transaction_payment::Config;
use sp_runtime::Perquintill;

use uniarts_primitives::*;
pub use crate::constants::currency::*;
use crate::*;

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