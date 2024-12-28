//! Rustic Coffee Shop.
//!
//! This is a framework for a container image to be hosted on AWS. It consists of the
//! following components:
//!
//! - Waiter - The Axum HTTP host serving incoming requests. The requests are then put
//!   into an AWS SQS standard queue, which will then

pub mod errors;
pub use errors::{CoffeeMachineError, CoffeeShopError};

pub mod helpers;
pub mod models;

pub mod cli;

mod logger;
