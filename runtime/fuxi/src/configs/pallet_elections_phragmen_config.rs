use crate::{weights::pallet_elections_phragmen::WeightInfo, *};
use pallet_elections_phragmen::Config;

frame_support::parameter_types! {
	pub const CandidacyBond: Balance = 10 * UART;
	// 1 storage item created, key size is 32 bytes, value size is 16+16.
	pub const VotingBondBase: Balance = deposit(1, 64);
	// additional data per vote is 32 bytes (account id).
	pub const VotingBondFactor: Balance = deposit(0, 32);
	pub const TermDuration: BlockNumber = 24 * HOURS;
	pub const DesiredMembers: u32 = 7;
	pub const DesiredRunnersUp: u32 = 7;
}

impl Config for Runtime {
    type Event = Event;
    type ModuleId = ElectionsPhragmenModuleId;
    type Currency = Uart;
    type ChangeMembers = Council;
    type InitializeMembers = Council;
    type CurrencyToVote = uniarts_common::currency::CurrencyToVoteHandler;
    type CandidacyBond = CandidacyBond;
    type VotingBondBase = VotingBondBase;
    type VotingBondFactor = VotingBondFactor;
    type LoserCandidate = Treasury;
    type KickedMember = Treasury;
    type DesiredMembers = DesiredMembers;
    type DesiredRunnersUp = DesiredRunnersUp;
    type TermDuration = TermDuration;
    type WeightInfo = WeightInfo<Runtime>;
}