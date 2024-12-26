//! Helper functions to transform any serializable struct into a binary payload before
//! compression. DynamoDB can natively store binary data.
//!
//! Currently, the chosen method is to use [`bincode`] for serialization and [`brotli`] for
//! compression.
//!
use bincode::Options;
use gzp::{
    deflate::Mgzip,
    par::{compress::ParCompress, decompress::ParDecompress},
    ZWriter,
};
use std::io::Write;
use tempfile::TempDir;

use super::buffer;

use crate::CoffeeShopError;

#[cfg(feature = "debug")]
const LOG_TARGET: &str = "coffeeshop::helpers::serde";

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
pub async fn serialize<'d, O: serde::Serialize>(
    data: &O,
    temp_dir: &'d TempDir,
) -> Result<buffer::BufferOnDisk<'d, buffer::Read>, CoffeeShopError> {
    // Create a new file buffer to store the compressed data.
    let mut buffer = buffer::BufferOnDisk::new(temp_dir, Some("serialize-gzp-")).await?;

    {
        // Use all available CPU cores for compression. `num_cpus` will return the number
        // of threads available on the system, i.e. we do not need to multiply this by 2
        // for Hyper-Threading.
        let thread_count = num_cpus::get();

        // ParCompress requires the Writer to be 'static, and it provides no access
        // to the underlying writer once built. So we have to use a NamedTempFile to
        // store the compressed data on disk, then read it back into memory.
        let mut cwriter = ParCompress::<Mgzip>::builder()
            .compression_level(gzp::Compression::new(6))
            .num_threads(thread_count)
            .map_err(CoffeeShopError::ResultBinaryCompressionError)?
            .from_writer(buffer.writer()?);

        let bincode_options = bincode_options_builder();

        // Serialize the data into the buffer.
        #[cfg(feature = "debug")]
        let start = tokio::time::Instant::now();

        bincode_options
            .serialize_into(&mut cwriter, data)
            .map_err(CoffeeShopError::ResultBinaryConversionError)?;

        // Flush the buffer.
        cwriter.flush().map_err(CoffeeShopError::from_io_error)?;

        // Flush the buffer.
        cwriter
            .finish()
            .map_err(CoffeeShopError::ResultBinaryCompressionError)?;

        #[cfg(feature = "debug")]
        crate::debug!(
            target: LOG_TARGET,
            "Serialization completed in {:?} using {} threads.",
            start.elapsed(),
            thread_count,
        );
    }

    buffer.finish().await
}

/// Deserialize a binary payload into a struct.
pub fn deserialize<O: serde::de::DeserializeOwned>(data: Vec<u8>) -> Result<O, CoffeeShopError> {
    let reader = std::io::Cursor::new(data);
    let mut creader = ParDecompress::<Mgzip>::builder().from_reader(reader);

    let bincode_options = bincode_options_builder();

    let result = bincode_options
        .deserialize_from(&mut creader)
        .map_err(CoffeeShopError::ResultBinaryConversionError)?;

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
                    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
                    let temp_file: buffer::BufferOnDisk<'_, _> = serialize(&data, &temp_dir).await.expect("Failed to serialize data");

                    let result = temp_file.read_to_end().await.expect("Failed to read serialized data");

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
            expected_head=[31, 139, 8, 4, 0], expected_tail=[0, 0, 0, 0, 0], expected_len=72
        ));

        create_test!(serialize_vec_string(
            input = vec!["hello".to_string(), "world".to_string()],
            output_type = Vec<String>,
            expected_head=[31, 139, 8, 4, 0], expected_tail=[0, 0, 0, 0, 0], expected_len=79
        ));

        create_test!(serialize_long_vec_u32(
            input = vec![u32::MAX; 65536],
            output_type = Vec<u32>,
            expected_head=[31, 139, 8, 4, 0], expected_tail=[0, 0, 0, 0, 0], expected_len=653
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

                    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
                    let temp_file: buffer::BufferOnDisk<'_, _> = serialize(&data, &temp_dir).await.expect("Failed to serialize data");

                    let serialized = temp_file.read_to_end().await.expect("Failed to read serialized data");
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
        create_test!(serialize_f64_10m(f64, 10 * 1024 * 1024));
    }
}
