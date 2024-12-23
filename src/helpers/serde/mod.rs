//! Helper functions to transform any serializable struct into a binary payload before
//! compression. DynamoDB can natively store binary data.
//!
//! Currently, the chosen method is to use [`bincode`] for serialization and [`brotli`] for
//! compression.
//!

use std::io::{BufWriter, Write};

use bincode::Options;

use crate::CoffeeShopError;

/// The compression level to use when compressing the payload.
pub const COMPRESSION_LEVEL: u32 = 11;

/// The window size to use when compressing the payload.
pub const WINDOW_SIZE: u32 = 22;

/// The buffer size to use when compressing the payload.
pub const BUFFER_SIZE: usize = 1024 * 1024;

/// The default options for bincode serialization.
pub fn bincode_options_builder() -> impl bincode::config::Options {
    bincode::DefaultOptions::new()
        .with_big_endian()
        .with_varint_encoding()
}

/// Serialize a struct into a binary payload.
pub fn serialize<O: serde::Serialize>(data: &O) -> Result<Vec<u8>, CoffeeShopError> {
    let mut output = Vec::with_capacity(BUFFER_SIZE * 16);

    {
        let writer = BufWriter::with_capacity(BUFFER_SIZE, &mut output);

        // Wrap the writer in a brotli compressor.
        let mut cwriter =
            brotli::CompressorWriter::new(writer, BUFFER_SIZE, COMPRESSION_LEVEL, WINDOW_SIZE);

        let bincode_options = bincode_options_builder();

        // Serialize the data into the buffer.
        bincode_options
            .serialize_into(&mut cwriter, data)
            .map_err(CoffeeShopError::ResultBinaryConversionError)?;

        // Flush the buffer.
        cwriter.flush().map_err(CoffeeShopError::IOError)?;

        Ok::<_, CoffeeShopError>(())
    }?;

    Ok(output)
}

/// Deserialize a binary payload into a struct.
pub fn deserialize<O: serde::de::DeserializeOwned>(data: &[u8]) -> Result<O, CoffeeShopError> {
    let mut reader = brotli::Decompressor::new(data, BUFFER_SIZE);

    let bincode_options = bincode_options_builder();

    let result = bincode_options
        .deserialize_from(&mut reader)
        .map_err(CoffeeShopError::ResultBinaryConversionError)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod primitives {
        use std::u32;

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
                #[test]
                fn $name() {
                    let data = $input;
                    let result: Vec<u8> = serialize(&data).expect("Failed to serialize data");

                    println!("Testing serialization: name={:?}, expected_head={:?}, expected_tail={:?}, expected_len={:?}", stringify!($name), &result[..5], &result[(result.len()-5)..], result.len());
                    assert_eq!(&result[..5], $expected_head);
                    assert_eq!(&result[(result.len()-5)..], $expected_tail);
                    assert_eq!(result.len(), $expected_len);

                    let deserialized = deserialize::<$output_type>(&result).expect("Failed to deserialize data");
                    assert_eq!(data, deserialized);
                }
            };
        }

        create_test!(serialize_vec_u32(
            input = vec![1_u32, 2, 3, 4, 5],
            output_type = Vec<u32>,
            expected_head = [139, 2, 128, 5, 1],
            expected_tail = [2, 3, 4, 5, 3],
            expected_len = 10
        ));

        create_test!(serialize_vec_string(
            input = vec!["hello".to_string(), "world".to_string()],
            output_type = Vec<String>,
            expected_head = [11, 6, 128, 2, 5],
            expected_tail = [111, 114, 108, 100, 3],
            expected_len = 17
        ));

        create_test!(serialize_long_vec_u32(
            input = vec![u32::MAX; 65536],
            output_type = Vec<u32>,
            expected_head = [43, 2, 128, 130, 255],
            expected_tail = [130, 46, 48, 0, 3],
            expected_len = 25
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
                #[test]
                fn $name() {
                    // get some random data:
                    let mut data: Vec<$dtype> = vec![<$dtype as Default>::default(); $size];
                    rand::thread_rng().fill(data.as_mut_slice());

                    let serialized: Vec<u8> = serialize(&data).expect("Failed to serialize data");
                    let serialized_json = serde_json::to_string(&data).expect("Failed to serialize data to JSON");
                    let serialized_binary = bincode::serialize(&data).expect("Failed to serialize data to binary");
                    assert_ne!(serialized.len(), 0);
                    // JSON is always larger than binary; if we can't even make this assertion, something is very wrong.
                    assert!(serialized_json.len() > serialized.len());
                    // Test if compression had worked.
                    assert!(serialized_binary.len() > serialized.len());
                    println!("Testing random serialization: name={:?}, len={}, serialied_size={}, final_size={}, json_size={}", stringify!($name), data.len(), serialized_binary.len(), serialized.len(), serialized_json.len());
                    let deserialized = deserialize::<Vec<$dtype>>(&serialized).expect("Failed to deserialize data");

                    assert_eq!(data, deserialized);
                }
            };
        }

        create_test!(serialize_f64_1(f64, 1));
        create_test!(serialize_f64_2(f64, 2));
        create_test!(serialize_f64_8(f64, 8));
        create_test!(serialize_f64_1k(f64, 1024));
        // create_test!(serialize_f64_1m(f64, 1024 * 1024));
        // create_test!(serialize_f64_10m(f64, 10 * 1024 * 1024));
    }
}
