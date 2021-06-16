pub mod currency {
    use uniarts_primitives::Balance;

    pub const DOLLARS: Balance = 1_000_000;
    pub const CENTS: Balance = DOLLARS / 100;
    pub const MILLICENTS: Balance = DOLLARS / 1_000;

    pub const NANO: Balance = 1;
    pub const MICRO: Balance = 1_000 * NANO;
    pub const MILLI: Balance = 1_000 * MICRO;
    pub const BILLI: Balance = 1_000 * MILLI;
    pub const UART: Balance = 1_000 * BILLI;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 20 * UART + (bytes as Balance) * 100 * MICRO
    }
}

pub mod time {
    use uniarts_primitives::{Moment, BlockNumber};

    pub const MILLISECS_PER_BLOCK: Moment = 6000;

    pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

    // Time is measured by number of blocks.
    pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
    pub const HOURS: BlockNumber = MINUTES * 60;
    pub const DAYS: BlockNumber = HOURS * 24;
}