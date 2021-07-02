use pallet_membership::Config;
use uniarts_primitives::*;
use crate::*;
use super::pallet_collective_config::CouncilInstance;

pub type EnsureRootOrMoreThanHalfCouncil = EnsureOneOf<
    AccountId,
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilInstance>,
>;

pub type ApproveOrigin = EnsureOneOf<
    AccountId,
    EnsureRoot<AccountId>,
    pallet_collective::EnsureProportionAtLeast<_3, _5, AccountId, CouncilInstance>,
>;

pub struct MembershipChangedGroup;
impl ChangeMembers<AccountId> for MembershipChangedGroup {
    fn change_members_sorted(
        incoming: &[AccountId],
        outgoing: &[AccountId],
        sorted_new: &[AccountId],
    ) {
        TechnicalCommittee::change_members_sorted(incoming, outgoing, sorted_new);
    }
}

pub type CouncilMembershipInstance = pallet_membership::Instance0;
impl Config<CouncilMembershipInstance> for Runtime {
    type Event = Event;
    type AddOrigin = EnsureRootOrMoreThanHalfCouncil;
    type RemoveOrigin = EnsureRootOrMoreThanHalfCouncil;
    type SwapOrigin = EnsureRootOrMoreThanHalfCouncil;
    type ResetOrigin = EnsureRootOrMoreThanHalfCouncil;
    type PrimeOrigin = EnsureRootOrMoreThanHalfCouncil;
    type MembershipInitialized = Council;
    type MembershipChanged = Council;
}

pub type TechnicalCommitteeMembershipInstance = pallet_membership::Instance1;
impl Config<TechnicalCommitteeMembershipInstance> for Runtime {
    type Event = Event;
    type AddOrigin = EnsureRootOrMoreThanHalfCouncil;
    type RemoveOrigin = EnsureRootOrMoreThanHalfCouncil;
    type SwapOrigin = EnsureRootOrMoreThanHalfCouncil;
    type ResetOrigin = EnsureRootOrMoreThanHalfCouncil;
    type PrimeOrigin = EnsureRootOrMoreThanHalfCouncil;
    type MembershipInitialized = TechnicalCommittee;
    type MembershipChanged = MembershipChangedGroup;
}