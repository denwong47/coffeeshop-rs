//! Helper functions to transform any serializable struct into a binary payload before
//! compression. DynamoDB can natively store binary data.
//!
//! Currently, the chosen method is to use [`bincode`] for serialization and [`lzma`] for
//! compression.
//!
use bincode::Options;
use lzma::{LzmaReader, LzmaWriter};
use std::io::Write;

use crate::{models::message::ProcessResult, CoffeeShopError};

#[cfg(feature = "debug")]
#[allow(dead_code)]
const LOG_TARGET: &str = "coffeeshop::helpers::serde";

/// The compression level to use when compressing the payload.
pub const COMPRESSION_LEVEL: u32 = 11;

/// The window size to use when compressing the payload.
pub const WINDOW_SIZE: u32 = 22;

/// The buffer size to use when compressing the payload.
pub const BUFFER_SIZE: usize = 1024 * 1024;

/// The preset to use when compressing the payload.
pub const LZMA_PRESET: u32 = 9;

/// The default options for bincode serialization.
pub fn bincode_options_builder() -> impl bincode::config::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_varint_encoding()
}

/// Serialize a struct into a binary payload using a writer.
///
/// The [`LzmaWriter`] will be consumed, and the compressed data will be returned.
/// This function is useful when you want to serialize a struct into a buffer.
///
/// Private function.
async fn serialize_using<O: serde::Serialize + Send + Sync + 'static>(
    data: O,
    mut writer: LzmaWriter<Vec<u8>>,
) -> Result<Vec<u8>, CoffeeShopError> {
    let bincode_options = bincode_options_builder();

    tokio::task::spawn_blocking(move || {
        bincode_options.serialize_into(&mut writer, &data).map_err(
            // If the error had been forced into a `std::io::ErrorType::Other`,
            // then we can assume the error came from the `writer`.
            // In our case the `writer` can emit a `LzmaError::MemLimit`,
            // which we will want to map to an oversize error.
            CoffeeShopError::BinaryConversionError,
        )?;

        writer.flush().map_err(CoffeeShopError::from_io_error)?;
        writer
            .finish()
            .map_err(CoffeeShopError::BinaryCompressionError)
    })
    .await
    .map_err(|err| CoffeeShopError::ThreadResourceError(err.to_string()))?
}

/// Serialize a struct into a binary payload.
pub async fn serialize<O: serde::Serialize + Send + Sync + 'static>(
    data: O,
) -> Result<Vec<u8>, CoffeeShopError> {
    let buffer = Vec::new();
    let writer = LzmaWriter::new_compressor(buffer, LZMA_PRESET)?;

    serialize_using(data, writer).await
}

/// Serialize a struct into a binary payload with an upper limited size.
/// If the serialized data is larger than the limit, an error will be returned.
///
/// The limit is in bytes.
///
/// # Note
///
/// If you intend to encode the data after this function, you should consider the size
/// may expand after encoding, and that this limit may not be sufficient as the only
/// check.
pub async fn serialize_with_limit<O: serde::Serialize + Send + Sync + 'static>(
    data: O,
    limit: usize,
) -> Result<Vec<u8>, CoffeeShopError> {
    let buffer = Vec::new();
    let writer = LzmaWriter::with_capacity(limit, buffer, lzma::Direction::Compress, LZMA_PRESET)
        .map_err(CoffeeShopError::BinaryCompressionError)?;

    serialize_using(data, writer).await
}

/// Deserialize a binary payload into a struct.
pub fn deserialize<O: serde::de::DeserializeOwned + Sync + Send + 'static>(
    data: Vec<u8>,
) -> ProcessResult<O> {
    let reader = std::io::Cursor::new(data);
    let mut creader =
        LzmaReader::new_decompressor(reader).map_err(CoffeeShopError::BinaryCompressionError)?;

    let bincode_options = bincode_options_builder();

    let result = bincode_options
        .deserialize_from(&mut creader)
        .map_err(CoffeeShopError::BinaryConversionError)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "debug")]
    const LOG_TARGET: &str = "coffeeshop::helpers::serde::tests";

    mod primitives {
        use super::*;

        macro_rules! create_test {
            (
                $name:ident(
                    input=$input:expr,
                    output_type=$output_type:ty,
                    expected_head=$expected_head:expr,
                    expected_tail=$expected_tail:expr,
                    expected_len=$expected_len:expr
                )
            ) => {
                #[tokio::test]
                async fn $name() {
                    let data = $input;
                    let result = serialize(data.clone()).await.expect("Failed to serialize data");

                    crate::debug!(target: LOG_TARGET, "Testing serialization: name={:?}, expected_head={:?}, expected_tail={:?}, expected_len={:?}", stringify!($name), &result[..5], &result[(result.len()-5)..], result.len());
                    assert_eq!(&result[..5], $expected_head);
                    assert_eq!(&result[(result.len()-5)..], $expected_tail);
                    assert_eq!(result.len(), $expected_len);

                    let deserialized = deserialize::<$output_type>(result).expect("Failed to deserialize data");
                    assert_eq!(data, deserialized);
                }
            };
        }

        create_test!(serialize_vec_u32(
            input = vec![1_u32, 2, 3, 4, 5],
            output_type = Vec<u32>,
            expected_head=[253, 55, 122, 88, 90], expected_tail=[0, 0, 4, 89, 90], expected_len=64
        ));

        create_test!(serialize_vec_string(
            input = vec!["hello".to_string(), "world".to_string()],
            output_type = Vec<String>,
            expected_head=[253, 55, 122, 88, 90], expected_tail=[0, 0, 4, 89, 90], expected_len=72
        ));

        create_test!(serialize_long_vec_u32(
            input = vec![u32::MAX; 65536],
            output_type = Vec<u32>,
            expected_head=[253, 55, 122, 88, 90], expected_tail=[0, 0, 4, 89, 90], expected_len=192
        ));
    }

    mod long_data {
        use super::*;
        use rand::Rng;

        macro_rules! create_test {
            (
                $name:ident(
                    $dtype:ty,
                    $size:expr
                )
            ) => {
                #[tokio::test]
                async fn $name() {
                    // get some random data:
                    let mut data: Vec<$dtype> = vec![<$dtype as Default>::default(); $size];
                    rand::thread_rng().fill(data.as_mut_slice());

                    let serialized = serialize(data.clone()).await.expect("Failed to serialize data");

                    let serialized_json = serde_json::to_string(&data).expect("Failed to serialize data to JSON");
                    let serialized_binary = bincode::serialize(&data).expect("Failed to serialize data to binary");
                    assert_ne!(serialized.len(), 0);
                    // JSON is always larger than binary unless the data is very small.
                    assert!(serialized_json.len().max(200) >= serialized.len(), "JSON size={}, compressed size={}", serialized_json.len(), serialized.len());
                    // Test if compression had worked.
                    assert!(serialized_binary.len().max(200) >= serialized.len(), "Binary size={}, compressed size={}", serialized_binary.len(), serialized.len());
                    crate::debug!(target: LOG_TARGET, "Testing random serialization: name={:?}, len={}, serialied_size={}, final_size={}, json_size={}", stringify!($name), data.len(), serialized_binary.len(), serialized.len(), serialized_json.len());
                    let deserialized = deserialize::<Vec<$dtype>>(serialized).expect("Failed to deserialize data");

                    assert_eq!(data, deserialized);
                }
            };
        }

        create_test!(serialize_f64_1(f64, 1));
        create_test!(serialize_f64_2(f64, 2));
        create_test!(serialize_f64_8(f64, 8));
        create_test!(serialize_f64_1k(f64, 1024));
        create_test!(serialize_f64_1m(f64, 1024 * 1024));
        // This is deemed not necessary as SQS has a limit of 256KB per message.
        // create_test!(serialize_f64_10m(f64, 10 * 1024 * 1024));
    }
}
