//! This module contains all the models used in the application.
//!
//! The primary models are:
//! - [`Shop`]: The single app instance that contains both the [`Barista`]s and
//!   the [`Waiter`].
//! - [`Barista`]: Workers that processes tickets.
//! - [`Waiter`]: The REST API host that serves incoming requests.
//! - [`Machine`]: The trait that defines the coffee machine that processes tickets;
//!   this is implemented by the user.
//! - [`Ticket`]: The request that is sent to the shop to be processed.
//! - [`MulticastMessage`](message::MulticastMessage): The gRPC message struct that is sent to all waiters
//!   in the same cluster to notify them of a finished ticket.
//! - [`Order`]: The struct that contains the processed ticket and the waiter
//!   notification.
//! - [`message`]: The module that contains the request and response structs for
//!   internal communication.

mod announcer;
pub use announcer::Announcer;

mod barista;
pub use barista::Barista;

mod order;
pub use order::{Order, OrderSegment, Orders};

mod shop;
pub use shop::*;

mod machine;
pub use machine::Machine;

mod waiter;
pub use waiter::*;

pub mod message;
pub use message::Ticket;

// Test models for testing the message module.
#[cfg(test)]
pub mod test;
