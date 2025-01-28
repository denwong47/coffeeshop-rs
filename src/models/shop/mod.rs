//! This module contains the models for the [`Shop`].

mod base;
pub use base::*;

mod open;
mod order;

mod implementations;
pub use implementations::*;

#[cfg(test)]
mod tests;
