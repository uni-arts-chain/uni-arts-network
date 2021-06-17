use uniarts_primitives::*;
use pallet_session::Config;
use super::pallet_validator_set_config::DisabledValidatorsThreshold;
use crate::*;
use crate::weights::pallet_session::WeightInfo;

pub struct ValidatorIdOf;
impl<T> Convert<T, Option<T>> for ValidatorIdOf {
    fn convert(a: T) -> Option<T> {
        Some(a)
    }
}

impl Config for Runtime {
    type Event = Event;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ValidatorIdOf;
    type ShouldEndSession = ValidatorSet;
    type NextSessionRotation = ValidatorSet;
    type SessionManager = ValidatorSet;
    type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = opaque::SessionKeys;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type WeightInfo = WeightInfo<Runtime>;
}