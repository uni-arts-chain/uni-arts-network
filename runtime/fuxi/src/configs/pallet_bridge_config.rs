use pallet_bridge::Config;
use uniarts_primitives::*;
use crate::*;

frame_support::parameter_types! {
	pub const GetBridgeCurrencyId: CurrencyId = CurrencyId::Token(TokenSymbol::USDT);
}

impl Config for Runtime {
    type Event = Event;
    type Currency = Currencies;
    type GetBridgeCurrencyId = GetBridgeCurrencyId;
}