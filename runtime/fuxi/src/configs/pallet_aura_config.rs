use pallet_aura::Config;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use crate::*;

impl Config for Runtime {
    type AuthorityId = AuraId;
}