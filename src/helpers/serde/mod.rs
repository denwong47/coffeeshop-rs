//! Helper functions to transform any serializable struct into a binary payload before
//! compression. DynamoDB can natively store binary data.
//!
//! Currently, the chosen method is to use [`bincode`] for serialization and [`brotli`] for
//! compression.
//!
