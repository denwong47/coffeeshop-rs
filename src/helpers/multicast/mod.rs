//! Multicast functions and structs for asynchronous communication among [`Shop`](crate::models::Shop) instances within the same cluster.

pub mod socket;

#[cfg(test)]
mod test;

/// The async socket type used in this crate.
pub use tokio_socket2::TokioSocket2 as AsyncSocket;
