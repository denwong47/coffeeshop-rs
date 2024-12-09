use thiserror::Error;

use std::net::IpAddr;

#[derive(Error, Debug)]
pub enum CoffeeShopError {
    #[error("Invalid configuration for {field}: {value}")]
    InvalidConfiguration { field: &'static str, value: String },

    #[error("{0:?} is not a valid multicast address.")]
    InvalidMulticastAddress(IpAddr),

    #[error("Received an invalid {field} in MulticastMessage: {value}")]
    InvalidMulticastMessage { field: &'static str, value: String },

    #[error("HTTP Host failed: {0}")]
    AxumError(axum::Error),
}
