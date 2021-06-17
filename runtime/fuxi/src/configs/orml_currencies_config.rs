use orml_currencies::Config;
use uniarts_primitives::*;
use crate::*;

frame_support::parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = CurrencyId::Native;
}

impl Config for Runtime {
    type Event = Event;
    type MultiCurrency = UniTokens;
    type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type WeightInfo = ();
}