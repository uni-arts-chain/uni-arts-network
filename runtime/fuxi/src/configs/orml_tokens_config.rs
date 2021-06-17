use orml_tokens::Config;
use uniarts_primitives::*;
use crate::*;

impl Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = CurrencyId;
    type OnReceived = ();
    type WeightInfo = ();
}