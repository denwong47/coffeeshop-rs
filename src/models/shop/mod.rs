//! This module contains the models for the [`Shop`].

mod base;
pub use base::*;

mod open;

mod implementations;
pub use implementations::*;

#[cfg(test)]
mod tests;
