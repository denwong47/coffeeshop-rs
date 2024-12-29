//! Rustic Coffee Shop.
//!
//! This is a framework for a container image to be hosted on AWS. It consists of the
//! following components:
//!
//! - Waiter - The Axum HTTP host serving incoming requests. The requests are then put
//!   into an AWS SQS standard queue, which will then

pub mod errors;
pub use errors::{CoffeeMachineError, CoffeeShopError};

pub mod reexports {
    #[cfg(doc)]
    use super::models;

    /// Re-export the `async_trait` crate so that implementors of [`models::Machine`]
    /// can use it without concerns for mismatched versions.
    pub use async_trait;
}

pub mod helpers;
pub mod models;

pub mod cli;

mod logger;
