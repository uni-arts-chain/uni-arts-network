// use pallet_nicks::Config;
// use crate::*;
//
// frame_support::parameter_types! {
//     // Choose a fee that incentivizes desireable behavior.
//     pub const NickReservationFee: u128 = 100;
//     pub const MinNickLength: usize = 6;
//     // Maximum bounds on storage are important to secure your chain.
//     pub const MaxNickLength: usize = 32;
// }
//
// impl Config for Runtime {
//     /// The Balances pallet implements the ReservableCurrency Config.
//     type Currency = Uart;
//     /// Use the NickReservationFee from the parameter_types block.
//     type ReservationFee = NickReservationFee;
//     /// No action is taken when deposits are forfeited.
//     type Slashed = Treasury;
//     /// Configure the FRAME System Root origin as the Nick pallet admin.
//     type ForceOrigin = frame_system::EnsureRoot<AccountId>;
//     /// Use the MinNickLength from the parameter_types block.
//     type MinLength = MinNickLength;
//     /// Use the MaxNickLength from the parameter_types block.
//     type MaxLength = MaxNickLength;
//     /// The ubiquitous event type.
//     type Event = Event;
// }