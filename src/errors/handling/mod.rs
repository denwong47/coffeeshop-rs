//! Error handling module.
//!
//! This contains the error handling logic for the application, including traits
//! and wrapper functions to deal with Deserialization errors at [`axum`] level.

/// For internal use.
use super::CoffeeShopError;

mod into_coffeeshop_error;
pub use into_coffeeshop_error::IntoCoffeeShopError;
