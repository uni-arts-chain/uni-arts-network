use orml_tokens::Config;
use uniarts_primitives::*;
use crate::*;
use crate::weights::orml_tokens::WeightInfo;

impl Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = orml_tokens::TransferDust<Runtime, UniArtsTreasuryModuleId>;
    type WeightInfo = WeightInfo<Runtime>;
}