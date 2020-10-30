pub mod currency {
    use primitives::Balance;

    pub const UART: Balance = 1_000_000_000_000;
    pub const DOLLARS: Balance = 100;
    pub const CENTS: Balance = DOLLARS / 100;
    pub const MILLICENTS: Balance = DOLLARS / 1_000;

    pub const NANO: Balance = 1;
    pub const MICRO: Balance = 1_000 * NANO;
    pub const MILLI: Balance = 1_000 * MICRO;
    pub const COIN: Balance = 1_000 * MILLI;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 20 * COIN + (bytes as Balance) * 100 * MICRO
    }
}