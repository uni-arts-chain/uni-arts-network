use pallet_scheduler::Config;
use uniarts_primitives::*;
use crate::{weights::pallet_scheduler::WeightInfo, *};

frame_support::parameter_types! {
	pub const MaxScheduledPerBlock: u32 = 50;
}
impl Config for Runtime {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumBlockWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = WeightInfo<Runtime>;
}