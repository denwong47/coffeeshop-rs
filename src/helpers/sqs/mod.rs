//! Helper functions for the [`Waiter`] to put [`Ticket`]s into the SQS queue,
//! and for the [`Barista`] to retrieve them.
//!

#[cfg(doc)]
use crate::models::{Barista, Waiter};

mod config;
pub use config::*;

pub mod encoding;

mod func;
pub use func::*;

mod staged_receipt;
pub use staged_receipt::*;

#[cfg(test)]
mod tests;
