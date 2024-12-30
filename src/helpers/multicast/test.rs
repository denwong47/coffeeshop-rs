//! Helper functions for testing the multicast module.

use std::net::{IpAddr, SocketAddr};

use crate::cli::Config;

/// Get the multicast address for testing.
pub fn get_multicast_addr() -> SocketAddr {
    match (
        std::env::var("MULTICAST_HOST"),
        std::env::var("MULTICAST_PORT"),
    ) {
        (Ok(host), Ok(port)) => {
            let host = host.parse().expect("Failed to parse the host address.");
            let port = port.parse().expect("Failed to parse the port number.");

            SocketAddr::new(IpAddr::V4(host), port)
        }
        _ => Config::default().multicast_addr(),
    }
}
