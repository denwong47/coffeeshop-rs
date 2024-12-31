//! Rustic Coffee Shop.
//!
//! This is a framework for a container image to be hosted on AWS. It consists of the
//! following components:
//!
//! - Waiter - The Axum HTTP host serving incoming requests. The requests are then put
//!   into an AWS SQS standard queue, which will then

pub mod errors;
pub use errors::{CoffeeMachineError, CoffeeShopError, ValidationError};

/// Re-export the necessary crates for implementors of [`models::Machine`].
///
/// This module is intended to be used by implementors of the [`models::Machine`] trait
/// to ensure that the versions of the compatible dependencies are accessible for the
/// downstream implementors.
pub mod reexports {
    #[cfg(doc)]
    use super::models;

    /// Re-export the `async_trait` crate so that implementors of [`models::Machine`]
    /// can use it without concerns for mismatched versions.
    pub use async_trait;

    /// Re-export the `axum` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use axum;

    /// Re-export the `serde` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use serde;

    /// Re-export the `serde_json` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use serde_json;

    /// Re-export the `uuid` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use uuid;

    /// Re-export the `tokio` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use socket2;

    /// Re-export the `tokio` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use tokio_socket2;
}

pub mod helpers;
pub mod models;

pub mod cli;

mod logger;
