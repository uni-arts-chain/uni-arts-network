use pallet_society::Config;
use uniarts_primitives::*;
use crate::*;
use crate::constants::{currency::*, time::*};

frame_support::parameter_types! {
	pub const CandidateDeposit: Balance = 10 * UART;
	pub const WrongSideDeduction: Balance = 2 * UART;
	pub const MaxStrikes: u32 = 10;
	pub const RotationPeriod: BlockNumber = 80 * HOURS;
	pub const PeriodSpend: Balance = 500 * UART;
	pub const MaxLockDuration: BlockNumber = 36 * 30 * DAYS;
	pub const ChallengePeriod: BlockNumber = 7 * DAYS;
}

impl Config for Runtime {
    type Event = Event;
    type ModuleId = SocietyModuleId;
    type Currency = Uart;
    type Randomness = RandomnessCollectiveFlip;
    type CandidateDeposit = CandidateDeposit;
    type WrongSideDeduction = WrongSideDeduction;
    type MaxStrikes = MaxStrikes;
    type PeriodSpend = PeriodSpend;
    type MembershipChanged = ();
    type RotationPeriod = RotationPeriod;
    type MaxLockDuration = MaxLockDuration;
    type FounderSetOrigin = EnsureRootOrMoreThanHalfCouncil;
    type SuspensionJudgementOrigin = pallet_society::EnsureFounder<Runtime>;
    type ChallengePeriod = ChallengePeriod;
}