use sp_runtime::{Percent, Permill};
use crate::{weights::pallet_treasury::WeightInfo, *};
use pallet_treasury::Config;
use crate::pallet_membership_config::{EnsureRootOrMoreThanHalfCouncil, ApproveOrigin};

frame_support::parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 10 * UART;
	pub const SpendPeriod: BlockNumber = 1 * DAYS;
	pub const Burn: Permill = Permill::from_percent(0);
	pub const TipCountdown: BlockNumber = 1 * DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(10);
	pub const TipReportDepositBase: Balance = 1 * UART;
	pub const SevenDays: BlockNumber = 7 * DAYS;
	pub const ZeroDay: BlockNumber = 0;
	pub const OneDay: BlockNumber = DAYS;
	pub const DataDepositPerByte: Balance = 1 * MILLI;
	pub const BountyDepositBase: Balance = 1 * UART;
	pub const BountyDepositPayoutDelay: BlockNumber = 4 * DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 90 * DAYS;
	pub const BountyCuratorDeposit: Permill = Permill::from_percent(50);
	pub const BountyValueMinimum: Balance = 10 * UART;
	pub const MaximumReasonLength: u32 = 16384;
}

impl Config for Runtime {
    type ModuleId = UniArtsTreasuryModuleId;
    type Currency = Uart;
    type ApproveOrigin = ApproveOrigin;
    type RejectOrigin = EnsureRootOrMoreThanHalfCouncil;
    type Event = Event;
    type OnSlash = Treasury;
    type ProposalBond = ProposalBond;
    type ProposalBondMinimum = ProposalBondMinimum;
    type SpendPeriod = SpendPeriod;
    type Burn = Burn;
    type BurnDestination = Society;
    type WeightInfo = WeightInfo<Runtime>;
}