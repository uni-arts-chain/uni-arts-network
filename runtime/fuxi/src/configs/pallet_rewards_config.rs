use crate::{weights::pallet_rewards::WeightInfo, *};
use uniarts_primitives::*;
use pallet_rewards::Config;
use pallet_staking;
use super::frame_system_config::BlocksPerDay;

pub use crate::constants::currency::*;
pub use crate::constants::time::*;

frame_support::parameter_types! {
	pub const MiningRewardPerBlock: Balance = 8 * UART;
	pub const RewardThreshold: Balance = 30 * (BlocksPerDay::get() as Balance) * MiningRewardPerBlock::get();
	pub const StakingRewardPerBlock: Balance = 1 * UART;
	pub const AmpFactor: Balance = 1e12 as Balance;
	pub const BlocksPerYear: u32 = 10; //365 * BlocksPerDay::get();
	pub const MiningCap: Balance = 150_000_000 * UART;
}

pub struct AccountIdOf;
impl<T> Convert<T, Option<T>> for AccountIdOf {
    fn convert(a: T) -> Option<T> {
        Some(a)
    }
}

pub struct ConvertNumberToBalance;
impl<BlockNumber, Balance: Bounded + core::convert::From<BlockNumber>> Convert<BlockNumber, Balance> for ConvertNumberToBalance {
    fn convert(a: BlockNumber) -> Balance {
        Balance::saturated_from::<BlockNumber>(a)
    }
}

impl Config for Runtime {
    type AccountIdOf = AccountIdOf;
    type Balance = Balance;
    type Currency = Uart;
    type RewardThreshold = RewardThreshold;
    type RewardPerBlock = MiningRewardPerBlock;
    type BlocksPerYear = BlocksPerYear;
    type MiningCap = MiningCap;
    type Event = Event;
    type WeightInfo = WeightInfo<Runtime>;
}

impl pallet_staking::Config for Runtime {
    type ModuleId = StakingModuleId;
    type Event = Event;
    type Currency = Uart;
    type RewardPerBlock = StakingRewardPerBlock;
    type Id = u32;
    type AmpFactor = AmpFactor;
    type ConvertNumberToBalance = ConvertNumberToBalance;
    type WeightInfo = ();
}