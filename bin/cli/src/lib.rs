//! Uniarts CLI library.
#![warn(missing_docs)]
#![warn(unused_extern_crates)]

#[cfg(feature = "cli")]
mod cli;
#[cfg(feature = "cli")]
mod command;

#[cfg(feature = "cli")]
pub use command::run;
#[cfg(feature = "cli")]
pub use sc_cli::Result;