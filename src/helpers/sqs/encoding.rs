//! Since AWS SQS does not permit binary payloads, it is necessary to serialize
//! the input into a string before sending it to the queue. This module provides
//! the necessary functions to serialize and deserialize the input.
//!
//! While the encoding and decoding itself is not asynchronous, it is possible
//! that we will require S3 to store oversized payloads in the future. Therefore
//! all functions are asynchronous to allow for future expansion.

use crate::CoffeeShopError;
use base64::Engine;

/// The size limit for a value in SQS messages.
pub const SIZE_LIMIT: usize = 256 * 1024;

/// The base64 encoder to use for encoding and decoding.
pub const BASE64_ENCODER: base64::engine::GeneralPurpose =
    base64::engine::general_purpose::STANDARD_NO_PAD;

/// Serialize a struct into a base64-encoded string.
pub async fn encode(data: &[u8]) -> Result<String, CoffeeShopError> {
    let result = BASE64_ENCODER.encode(data);

    if result.len() > SIZE_LIMIT {
        Err(CoffeeShopError::Base64EncodingOversize(result.len()))
    } else {
        Ok(result)
    }
}

/// Deserialize a base64-encoded string into a struct.
pub async fn decode(data: &str) -> Result<Vec<u8>, CoffeeShopError> {
    BASE64_ENCODER
        .decode(data.as_bytes())
        .map_err(CoffeeShopError::Base64DecodingError)
}
