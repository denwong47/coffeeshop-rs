#![feature(prelude_import)]
//! Rustic Coffee Shop.
//!
//! This is a framework for a container image to be hosted on AWS. It consists of the
//! following components:
//!
//! - Waiter - The Axum HTTP host serving incoming requests. The requests are then put
//!   into an AWS SQS standard queue, which will then
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod errors {
    use axum::{body::Body, http, response::IntoResponse, Json};
    use thiserror::Error;
    use std::net::IpAddr;
    pub enum CoffeeShopError {
        #[error("Invalid configuration for {field}: {value}")]
        InvalidConfiguration { field: &'static str, value: String },
        #[error("{0:?} is not a valid multicast address.")]
        InvalidMulticastAddress(IpAddr),
        #[error("Received an invalid {field} in MulticastMessage: {value}")]
        InvalidMulticastMessage { field: &'static str, value: String },
        #[error("HTTP Host failed: {0}")]
        AxumError(axum::Error),
        #[error("Could not serialize the payload: {0}")]
        ResultBinaryConversionError(#[from] Box<bincode::ErrorKind>),
        #[error("Could not compress/decompress the payload: {0}")]
        ResultBinaryCompressionError(#[from] gzp::GzpError),
        #[error("Temporary file access failure at {path}: {reason}")]
        TempFileAccessFailure { path: std::path::PathBuf, reason: String },
        #[error(
            "The path for a temporary file is non-uniquely generated; this is improbable unless cleanup is not working. Please verify."
        )]
        NonUniqueTemporaryFile,
        #[error("An IOError::{0} had occurred: {1}")]
        IOError(std::io::ErrorKind, std::io::Error),
        #[error("Timed out awaiting results after {0:?} seconds")]
        RetrieveTimeout(std::time::Duration),
        #[error("AWS Configuration incomplete: {0}")]
        AWSConfigIncomplete(String),
        #[error("AWS SDK Error: {0}")]
        AWSSdkError(String),
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::thiserror::__private::Error for CoffeeShopError {
        fn source(
            &self,
        ) -> ::core::option::Option<&(dyn ::thiserror::__private::Error + 'static)> {
            use ::thiserror::__private::AsDynError as _;
            #[allow(deprecated)]
            match self {
                CoffeeShopError::InvalidConfiguration { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::InvalidMulticastAddress { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::InvalidMulticastMessage { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::AxumError { .. } => ::core::option::Option::None,
                CoffeeShopError::ResultBinaryConversionError { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                CoffeeShopError::ResultBinaryCompressionError { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                CoffeeShopError::TempFileAccessFailure { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::NonUniqueTemporaryFile { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::IOError { .. } => ::core::option::Option::None,
                CoffeeShopError::RetrieveTimeout { .. } => ::core::option::Option::None,
                CoffeeShopError::AWSConfigIncomplete { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::AWSSdkError { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::core::fmt::Display for CoffeeShopError {
        fn fmt(&self, __formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            use ::thiserror::__private::AsDisplay as _;
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                CoffeeShopError::InvalidConfiguration { field, value } => {
                    match (field.as_display(), value.as_display()) {
                        (__display_field, __display_value) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid configuration for {0}: {1}",
                                        __display_field,
                                        __display_value,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::InvalidMulticastAddress(_0) => {
                    match (_0,) {
                        (__field0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "{0:?} is not a valid multicast address.",
                                        __field0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::InvalidMulticastMessage { field, value } => {
                    match (field.as_display(), value.as_display()) {
                        (__display_field, __display_value) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Received an invalid {0} in MulticastMessage: {1}",
                                        __display_field,
                                        __display_value,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::AxumError(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!("HTTP Host failed: {0}", __display0),
                                )
                        }
                    }
                }
                CoffeeShopError::ResultBinaryConversionError(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Could not serialize the payload: {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::ResultBinaryCompressionError(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Could not compress/decompress the payload: {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::TempFileAccessFailure { path, reason } => {
                    match (path.as_display(), reason.as_display()) {
                        (__display_path, __display_reason) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Temporary file access failure at {0}: {1}",
                                        __display_path,
                                        __display_reason,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::NonUniqueTemporaryFile {} => {
                    __formatter
                        .write_str(
                            "The path for a temporary file is non-uniquely generated; this is improbable unless cleanup is not working. Please verify.",
                        )
                }
                CoffeeShopError::IOError(_0, _1) => {
                    match (_0.as_display(), _1.as_display()) {
                        (__display0, __display1) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "An IOError::{0} had occurred: {1}",
                                        __display0,
                                        __display1,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::RetrieveTimeout(_0) => {
                    match (_0,) {
                        (__field0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Timed out awaiting results after {0:?} seconds",
                                        __field0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::AWSConfigIncomplete(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "AWS Configuration incomplete: {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::AWSSdkError(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(format_args!("AWS SDK Error: {0}", __display0))
                        }
                    }
                }
            }
        }
    }
    #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
    #[automatically_derived]
    impl ::core::convert::From<Box<bincode::ErrorKind>> for CoffeeShopError {
        fn from(source: Box<bincode::ErrorKind>) -> Self {
            CoffeeShopError::ResultBinaryConversionError {
                0: source,
            }
        }
    }
    #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
    #[automatically_derived]
    impl ::core::convert::From<gzp::GzpError> for CoffeeShopError {
        fn from(source: gzp::GzpError) -> Self {
            CoffeeShopError::ResultBinaryCompressionError {
                0: source,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CoffeeShopError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                CoffeeShopError::InvalidConfiguration {
                    field: __self_0,
                    value: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "InvalidConfiguration",
                        "field",
                        __self_0,
                        "value",
                        &__self_1,
                    )
                }
                CoffeeShopError::InvalidMulticastAddress(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidMulticastAddress",
                        &__self_0,
                    )
                }
                CoffeeShopError::InvalidMulticastMessage {
                    field: __self_0,
                    value: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "InvalidMulticastMessage",
                        "field",
                        __self_0,
                        "value",
                        &__self_1,
                    )
                }
                CoffeeShopError::AxumError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AxumError",
                        &__self_0,
                    )
                }
                CoffeeShopError::ResultBinaryConversionError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ResultBinaryConversionError",
                        &__self_0,
                    )
                }
                CoffeeShopError::ResultBinaryCompressionError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ResultBinaryCompressionError",
                        &__self_0,
                    )
                }
                CoffeeShopError::TempFileAccessFailure {
                    path: __self_0,
                    reason: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "TempFileAccessFailure",
                        "path",
                        __self_0,
                        "reason",
                        &__self_1,
                    )
                }
                CoffeeShopError::NonUniqueTemporaryFile => {
                    ::core::fmt::Formatter::write_str(f, "NonUniqueTemporaryFile")
                }
                CoffeeShopError::IOError(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "IOError",
                        __self_0,
                        &__self_1,
                    )
                }
                CoffeeShopError::RetrieveTimeout(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "RetrieveTimeout",
                        &__self_0,
                    )
                }
                CoffeeShopError::AWSConfigIncomplete(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AWSConfigIncomplete",
                        &__self_0,
                    )
                }
                CoffeeShopError::AWSSdkError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AWSSdkError",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl CoffeeShopError {
        /// Convenient method to create a [`CoffeeShopError::IOError`] variant from [`std::io::Error`].
        pub fn from_io_error(error: std::io::Error) -> Self {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                CoffeeShopError::NonUniqueTemporaryFile
            } else {
                CoffeeShopError::IOError(error.kind(), error)
            }
        }
        /// This method returns the appropriate HTTP status code for the error.
        ///
        /// Some of these errors will not be encountered as a result of a request,
        /// but are included for completeness.
        ///
        /// If not found, it will return a [`http::StatusCode::INTERNAL_SERVER_ERROR`].
        pub fn status_code(&self) -> http::StatusCode {
            match self {
                CoffeeShopError::InvalidMulticastAddress(_) => {
                    http::StatusCode::BAD_REQUEST
                }
                CoffeeShopError::InvalidMulticastMessage { .. } => {
                    http::StatusCode::BAD_REQUEST
                }
                CoffeeShopError::RetrieveTimeout(_) => http::StatusCode::REQUEST_TIMEOUT,
                _ => http::StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
        /// This method returns the kind of error as a string.
        pub fn kind(&self) -> String {
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("{0:?}", self));
                res
            })
        }
    }
    impl IntoResponse for CoffeeShopError {
        fn into_response(self) -> axum::response::Response<Body> {
            (
                self.status_code(),
                [
                    (http::header::CONTENT_TYPE, "application/json"),
                    (http::header::CACHE_CONTROL, "no-store"),
                ],
                Json(
                    ::serde_json::Value::Object({
                        let mut object = ::serde_json::Map::new();
                        let _ = object
                            .insert(
                                ("error").into(),
                                ::serde_json::to_value(&self.kind()).unwrap(),
                            );
                        let _ = object
                            .insert(
                                ("details").into(),
                                ::serde_json::Value::Object({
                                    let mut object = ::serde_json::Map::new();
                                    let _ = object
                                        .insert(
                                            ("message").into(),
                                            ::serde_json::to_value(&self.to_string()).unwrap(),
                                        );
                                    object
                                }),
                            );
                        object
                    }),
                ),
            )
                .into_response()
        }
    }
}
pub use errors::CoffeeShopError;
pub mod helpers {
    //! Helper functions for the Coffee Shop crate.
    //!
    pub mod aws {
        //! Centralized AWS helper functions.
        use crate::CoffeeShopError;
        /// Re-export the AWS configuration.
        pub use aws_config::SdkConfig;
        /// Get the AWS configuration from the environment variables or the provided arguments.
        ///
        /// # Note
        ///
        /// Currently this function only reads the configuration from the environment variables, and
        /// is always successful; however, in the future, it may be extended to read from a configuration
        /// file or other sources, which could fail.
        pub async fn get_aws_config() -> Result<aws_config::SdkConfig, CoffeeShopError> {
            let config = aws_config::load_from_env().await;
            Ok(config)
        }
    }
    pub mod buffer {
        use crate::CoffeeShopError;
        #[allow(unused_imports)]
        use std::{fs::File as StdFile, io::Write as StdWrite, path::PathBuf};
        use tempfile::TempDir;
        use tokio::{fs::File as TokioFile, io::AsyncReadExt};
        use uuid::Uuid;
        use super::serde::BUFFER_SIZE;
        /// The default prefix for the buffer if not specified.
        const DEFAULT_PREFIX: &str = "disk-buffer-";
        /// The default extension for the buffer if not specified.
        const DEFAULT_EXTENSION: &str = "bin";
        /// The default logger target for the buffer.
        #[cfg(feature = "debug")]
        const LOG_TARGET: &str = "coffeeshop::helpers::buffer";
        /// A file handler that can be either a standard file or a Tokio file.
        ///
        /// In our case, the write handler will be a blocking file handler, and the read
        /// handler will be an async file handler.
        pub enum FileHandler {
            /// Unused. [`Write`](StdWrite) handlers are not stored in the buffer,
            /// they are owned by the called of [`BufferOnDisk::writer`].
            Write(StdFile),
            /// The read handler for the buffer.
            ///
            /// Upon instantiating a [`BufferOnDisk`] in [`Read`] mode, a single
            /// read handler will be created and stored in the buffer.
            Read(TokioFile),
        }
        /// A trait to define the state of the buffer.
        pub trait BufferStateType {
            /// Get the state as a string.
            fn as_str(&self) -> &'static str;
        }
        /// Defines the state of the [`BufferOnDisk`].
        ///
        /// A [`BufferOnDisk`] needs to be written to first, before it can be read from.
        /// A [`Write`](Write) buffer can be transitioned to a
        /// [`Read`](Read) buffer, but not the other way around.
        pub struct Read {}
        impl BufferStateType for Read {
            fn as_str(&self) -> &'static str {
                "read"
            }
        }
        /// Defines the state of the [`BufferOnDisk`].
        ///
        /// A [`BufferOnDisk`] needs to be written to first, before it can be read from.
        /// A [`Write`](Write) buffer can be transitioned to a
        /// [`Read`](Read) buffer, but not the other way around.
        pub struct Write {}
        impl BufferStateType for Write {
            fn as_str(&self) -> &'static str {
                "write"
            }
        }
        /// A bytes buffer that is actually located on disk.
        ///
        /// This struct allows a buffer to be written [synchronously](StdWrite) and read
        /// [asynchronously](TokioFile), with the file being stored on disk in a
        /// [named file](Self::path).
        ///
        /// This is useful particularly for compression libraries which:
        ///
        /// - typically does not work asynchronously, and
        /// - may be required to process more data than can fit in memory.
        ///
        /// This buffer allows the compression to be streamed into disk, and then read
        /// at its own pace (e.g. sent over the network) without needing to keep
        /// potentially excessive amounts of data in memory.
        ///
        /// # Usage
        ///
        /// A [`BufferOnDisk`] can be instantiated with in the [`Write`](Write) state
        /// with a reference to a [`TempDir`]. The file will be created in the directory
        /// with a random UUID as the filename.
        ///
        /// Use the [`writer`](Self::writer) method to get a [`Write`](StdWrite) handler
        /// to write to the buffer. Once all data has been written, call
        /// [`finish`](Self::finish) to transition the buffer to the [`Read`](Read) state.
        ///
        /// The buffer can then be read using the [`reader`](Self::reader) method, or
        /// the whole buffer can be read into memory using the
        /// [`read_to_end`](Self::read_to_end) method.
        pub struct BufferOnDisk<'d, S: BufferStateType> {
            /// The directory to the buffer. This forces the temporary directory to be
            /// kept alive for the lifetime of the buffer.
            pub dir: &'d TempDir,
            /// The prefix for the buffer.
            pub prefix: String,
            /// The UUID of the file; this is randomly generated at the point of instantiation.
            pub uuid: Uuid,
            /// The [`FileHandler`] handle to the buffer.
            pub fhnd: Option<FileHandler>,
            _phantom: std::marker::PhantomData<S>,
        }
        impl<'d, S> BufferOnDisk<'d, S>
        where
            S: BufferStateType,
        {
            /// Get the path to the buffer.
            pub fn path(&self) -> PathBuf {
                self.dir
                    .path()
                    .join(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "{0}{1}.{2}",
                                    &self.prefix,
                                    &self.uuid,
                                    DEFAULT_EXTENSION,
                                ),
                            );
                            res
                        }),
                    )
            }
            /// Put the file handle into the buffer.
            pub fn with_file(mut self, fhnd: FileHandler) -> Self {
                self.fhnd = Some(fhnd);
                self
            }
            /// Take the file handle out of the buffer.
            pub fn take_file(&mut self) -> Option<FileHandler> {
                self.fhnd.take()
            }
            /// Touch the buffer to ensure it is writable.
            ///
            /// If the file already exists, an error will be returned.
            fn file_touch(&self) -> Result<(), CoffeeShopError> {
                std::fs::OpenOptions::new()
                    .create_new(true)
                    .write(true)
                    .open(self.path())
                    .inspect(|file| {
                        crate::logger::init();
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!("Touched file at {0:?} successfully.", file),
                                    lvl,
                                    &(
                                        LOG_TARGET,
                                        "coffeeshop::helpers::buffer",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        };
                    })
                    .and(Ok(()))
                    .map_err(CoffeeShopError::from_io_error)
            }
            /// Get a write handle to the buffer.
            ///
            /// If the file does not exist, it will be created.
            ///
            /// # Safety
            ///
            /// This method does not guard against multiple writers being given access to the
            /// buffer. It is up to the caller to ensure that only one writer is given access
            /// to the buffer at a time.
            fn file_write(&self) -> Result<StdFile, CoffeeShopError> {
                std::fs::OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(self.path())
                    .inspect(|file| {
                        crate::logger::init();
                        {
                            let lvl = ::log::Level::Trace;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!("Opened file at {0:?} for writing.", file),
                                    lvl,
                                    &(
                                        LOG_TARGET,
                                        "coffeeshop::helpers::buffer",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        };
                    })
                    .map_err(CoffeeShopError::from_io_error)
            }
            /// Get the buffer as a read handle.
            async fn file_read(&self) -> Result<TokioFile, CoffeeShopError> {
                TokioFile::open(self.path())
                    .await
                    .map_err(CoffeeShopError::from_io_error)
            }
        }
        /// Allow the buffer to be dropped safely by closing the file handle.
        impl<S> Drop for BufferOnDisk<'_, S>
        where
            S: BufferStateType,
        {
            fn drop(&mut self) {
                match self.take_file() {
                    Some(FileHandler::Read(file)) => {
                        drop(file);
                        std::fs::remove_file(self.path())
                            .unwrap_or_else(|err| {
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Debug;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Could not remove file at {0:?}, the temporary file will remain: {1:?}",
                                                    self.path(),
                                                    err,
                                                ),
                                                lvl,
                                                &(
                                                    LOG_TARGET,
                                                    "coffeeshop::helpers::buffer",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                }
                            });
                    }
                    Some(FileHandler::Write(file)) => {
                        drop(file);
                    }
                    None => {}
                }
            }
        }
        impl<'d> BufferOnDisk<'d, Write> {
            /// Create a new buffer on disk.
            ///
            /// The buffer will be created in the provided directory, with the provided
            /// prefix. If no prefix is provided, the default prefix will be used.
            ///
            /// If the file was not created, or it already exists, an error will be returned.
            pub async fn new(
                dir: &'d TempDir,
                prefix: Option<&str>,
            ) -> Result<Self, CoffeeShopError> {
                let prefix = prefix.unwrap_or(DEFAULT_PREFIX);
                let uuid = Uuid::new_v4();
                let instance = Self {
                    dir,
                    prefix: prefix.to_string(),
                    uuid,
                    fhnd: None,
                    _phantom: std::marker::PhantomData,
                };
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("Buffer created at {0:?}", instance.path()),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::helpers::buffer",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                instance.file_touch().map(|_| instance)
            }
            /// Complete the [`Write`](Write) state and transition to the
            /// [`Read`](Read) state.
            ///
            /// # Safety
            ///
            /// This method cannot ensure the [`Write`](StdWrite) is closed. Since the
            /// [`BufferOnDisk`] no longer has a reference to the [`Write`](StdWrite)
            /// handler, it cannot:
            ///
            /// - [`flush`](StdWrite::flush) the buffer, and
            /// - [`drop`](Drop::drop) the buffer to ensure the file is closed.
            ///
            /// It is up to the caller to ensure that the buffer is ready before calling this
            /// method.
            pub async fn finish(
                mut self,
            ) -> Result<BufferOnDisk<'d, Read>, CoffeeShopError> {
                let mut prefix = String::new();
                core::mem::swap(&mut self.prefix, &mut prefix);
                let instance = BufferOnDisk {
                    dir: self.dir,
                    prefix,
                    uuid: self.uuid,
                    fhnd: None,
                    _phantom: std::marker::PhantomData,
                };
                instance
                    .file_read()
                    .await
                    .map(|file| instance.with_file(FileHandler::Read(file)))
            }
            /// Return a [`Write`](StdWrite) handler for the buffer.
            ///
            /// Since [`BufferOnDisk`] itself does not implement [`Write`](StdWrite), this method
            /// is necessary to allow the caller to write to the buffer.
            ///
            /// This requires the caller to hold a mutable reference to the buffer, thus ensuring
            /// that only one writer is given access to the buffer at a time.
            ///
            /// The benefit of this approach is that the [`File`](std::fs::File) handle is wholly
            /// owned by the caller, and thus can be used in functions that require a
            /// `impl `[`Write`]` + 'static` trait bound, such as the
            /// [`gzp`](gzp::par::compress::ParCompressBuilder::from_writer) crate.
            ///
            /// # Safety
            ///
            /// However this method cannot ensure the [`Write`](StdWrite) is closed by the
            /// time [`finish`](Self::finish) is called. Since the [`BufferOnDisk`] no longer
            /// has a reference to the [`Write`](StdWrite) handler, it cannot:
            ///
            /// - [`flush`](StdWrite::flush) the buffer, and
            /// - [`drop`](Drop::drop) the buffer to ensure the file is closed.
            ///
            /// It is up to the caller to ensure that the buffer is ready to be
            /// [`finish`](Self::finish)ed.
            pub fn writer(&mut self) -> Result<StdFile, CoffeeShopError> {
                self.file_write()
            }
        }
        impl<'d> BufferOnDisk<'d, Read> {
            /// Create a new reader for the buffer.
            pub async fn reader(&mut self) -> Result<&mut TokioFile, CoffeeShopError> {
                let path = self.path();
                if let Some(FileHandler::Read(file)) = self.fhnd.as_mut() {
                    Ok(file)
                } else {
                    Err(CoffeeShopError::TempFileAccessFailure {
                        path,
                        reason: "The file handle is available for reading.".to_string(),
                    })
                }
            }
            /// Convenient method to read the whole buffer into memory.
            ///
            /// This will consume the buffer and the temporary file will be deleted.
            ///
            /// # Warning
            ///
            /// This method will read the whole buffer into memory. If the buffer is
            /// large, this may cause memory issues.
            ///
            /// It is recommended to use the [`reader`](Self::reader) method to read the
            /// buffer in chunks.
            ///
            /// This method is typically used in unit tests only.
            pub async fn read_to_end(mut self) -> Result<Vec<u8>, CoffeeShopError> {
                let reader = self.reader().await?;
                let mut output = Vec::with_capacity(BUFFER_SIZE);
                reader
                    .read_to_end(&mut output)
                    .await
                    .and(Ok(output))
                    .map_err(CoffeeShopError::from_io_error)
            }
        }
    }
    pub mod dynamodb {
        //! Helper functions for the [`Shop`](crate::models::Shop) to put processed results into
        //! DynamoDB.
        //!
    }
    pub mod multicast {
        //! Multicast functions and structs for asynchronous communication among [`Shop`](crate::models::Shop) instances within the same cluster.
    }
    pub mod serde {
        //! Helper functions to transform any serializable struct into a binary payload before
        //! compression. DynamoDB can natively store binary data.
        //!
        //! Currently, the chosen method is to use [`bincode`] for serialization and [`brotli`] for
        //! compression.
        //!
        use bincode::Options;
        use gzp::{
            deflate::Mgzip, par::{compress::ParCompress, decompress::ParDecompress},
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
            bincode::DefaultOptions::new().with_big_endian().with_varint_encoding()
        }
        /// Serialize a struct into a binary payload.
        pub async fn serialize<'d, O: serde::Serialize>(
            data: &O,
            temp_dir: &'d TempDir,
        ) -> Result<buffer::BufferOnDisk<'d, buffer::Read>, CoffeeShopError> {
            let mut buffer = buffer::BufferOnDisk::new(temp_dir, Some("serialize-gzp-"))
                .await?;
            {
                let thread_count = num_cpus::get();
                let mut cwriter = ParCompress::<Mgzip>::builder()
                    .compression_level(gzp::Compression::new(6))
                    .num_threads(thread_count)
                    .map_err(CoffeeShopError::ResultBinaryCompressionError)?
                    .from_writer(buffer.writer()?);
                let bincode_options = bincode_options_builder();
                #[cfg(feature = "debug")]
                let start = tokio::time::Instant::now();
                bincode_options
                    .serialize_into(&mut cwriter, data)
                    .map_err(CoffeeShopError::ResultBinaryConversionError)?;
                cwriter.flush().map_err(CoffeeShopError::from_io_error)?;
                cwriter.finish().map_err(CoffeeShopError::ResultBinaryCompressionError)?;
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!(
                                    "Serialization completed in {0:?} using {1} threads.",
                                    start.elapsed(),
                                    thread_count,
                                ),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::helpers::serde",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
            }
            buffer.finish().await
        }
        /// Deserialize a binary payload into a struct.
        pub fn deserialize<O: serde::de::DeserializeOwned>(
            data: Vec<u8>,
        ) -> Result<O, CoffeeShopError> {
            let reader = std::io::Cursor::new(data);
            let mut creader = ParDecompress::<Mgzip>::builder().from_reader(reader);
            let bincode_options = bincode_options_builder();
            let result = bincode_options
                .deserialize_from(&mut creader)
                .map_err(CoffeeShopError::ResultBinaryConversionError)?;
            Ok(result)
        }
    }
    pub mod sqs {
        //! Helper functions for the [`Waiter`] to put [`Ticket`]s into the SQS queue,
        //! and for the [`Barista`] to retrieve them.
        //!
        use aws_sdk_sqs as sqs;
        use crate::{
            models::{message, Ticket},
            CoffeeShopError,
        };
        use super::aws;
        /// Put a ticket into the AWS SQS queue.
        pub async fn put_ticket<Q, I>(
            queue: &str,
            input: message::CombinedInput<Q, I>,
            config: &aws::SdkConfig,
        ) -> Result<Ticket, CoffeeShopError>
        where
            Q: message::QueryType,
            I: serde::de::DeserializeOwned + serde::Serialize,
        {
            let client = aws_sdk_sqs::Client::new(config);
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub mod sts {
        //! Helper functions to confirm that the user has logged in with the correct credentials.
        //!
        use crate::CoffeeShopError;
        pub use aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityOutput as AWSCallerIdentity;
        use super::aws;
        #[cfg(feature = "debug")]
        const LOG_TARGET: &str = "coffeeshop::helpers::sts";
        /// Confirm that the user has logged in with the correct credentials.
        pub async fn get_aws_login(
            config: Option<&aws_config::SdkConfig>,
        ) -> Result<AWSCallerIdentity, CoffeeShopError> {
            let config = if let Some(config) = config {
                config
            } else {
                &aws::get_aws_config().await?
            };
            {
                crate::logger::init();
                {
                    let lvl = ::log::Level::Trace;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "Attempting to get STS caller identity with configuration: {0:?}",
                                config,
                            ),
                            lvl,
                            &(
                                LOG_TARGET,
                                "coffeeshop::helpers::sts",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
            };
            let client = aws_sdk_sts::Client::new(config);
            client
                .get_caller_identity()
                .send()
                .await
                .map_err(|err| CoffeeShopError::AWSSdkError(err.to_string()))
        }
        /// Report the AWS caller identity.
        pub async fn report_aws_login(
            config: Option<&aws_config::SdkConfig>,
        ) -> Result<(), CoffeeShopError> {
            let identity = get_aws_login(config).await?;
            {
                crate::logger::init();
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "AWS credentials: UserId: {0:?}, Account: {1:?}, Arn: {2:?}",
                                identity.user_id.unwrap_or("(none)".to_string()),
                                identity.account.unwrap_or("(none)".to_string()),
                                identity.arn.unwrap_or("(none)".to_string()),
                            ),
                            lvl,
                            &(
                                LOG_TARGET,
                                "coffeeshop::helpers::sts",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
            };
            Ok(())
        }
    }
}
pub mod models {
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
    //! - [`MulticastMessage`]: The gRPC message struct that is sent to all waiters
    //!   in the same cluster to notify them of a finished ticket.
    //! - [`message`]: The module that contains the request and response structs for
    //!   internal communication.
    mod barista {
        use std::sync::{atomic::AtomicUsize, Arc};
        use super::{Machine, Shop};
        /// A [`Barista`] instance that acts as a worker for the shop.
        ///
        /// A shop can have any positive number of [`Barista`] instances; they are responsible
        /// for taking [`Ticket`]s from the SQS queue, process them, and send the results to
        /// DynamoDB with the [`Ticket`] being the key.
        ///
        /// They are also responsible for sending a multicast message to all the waiters in
        /// the same cluster (including those in different [`Shop`]s), so that the waiters can
        /// retrieve the results when ready instead of polling the DynamoDB table.
        pub struct Barista<I, O, F>
        where
            I: serde::de::DeserializeOwned + serde::Serialize,
            O: serde::Serialize + serde::de::DeserializeOwned,
            F: Machine<I, O>,
        {
            /// A back reference to the shop that this barista is serving.
            pub shop: Arc<Shop<I, O, F>>,
            /// The total amount of historical requests processed.
            pub process_count: AtomicUsize,
        }
    }
    pub use barista::Barista;
    mod shop {
        #![allow(dead_code)]
        use hashbrown::HashMap;
        use serde::{de::DeserializeOwned, Serialize};
        use std::{marker::PhantomData, sync::Arc};
        use tokio::sync::{Notify, RwLock};
        use super::Machine;
        use crate::{cli::Config, helpers, CoffeeShopError};
        /// The default prefix for dynamodb table.
        const DYNAMODB_TABLE_PREFIX: &str = "task-queue-";
        /// A coffee shop that has a waiter to take orders, and positive number of baristas to process
        /// tickets using the coffee machine.
        ///
        /// The shop is expected to:
        /// - Listen for incoming requests,
        /// - Convert the requests into tickets on a shared AWS SQS queue,
        /// - Have baristas to process the tickets using the coffee machine,
        /// - Put the finished coffee into a AWS DynamoDB table using the SQS id as the key, then
        /// - The barista will shout out the ticket number for the waiter to pick up the order.
        ///
        /// The [`Shop`] is designed to work with load balancers and auto-scaling groups, so that more
        /// [`Shop`] instances can be deployed to the same cluster to handle the same
        /// queue, without dropping any messages. The load balancing can be performed on the
        /// number of messages in the queue.
        ///
        /// Depending on the node type for the [`Shop`], each
        /// [`Shop`] can have a different number of barristas within it, but will always have one
        /// waiter. Choosing the waiter to serve incoming requests is the responsibility of the
        /// load balancer, and is not part of this implementation; however as the waiter has
        /// very virtually no blocking work to do, [`tokio`] alone should be able to handle
        /// a large number of requests even if they are not perfectly balanced across [`Shop`]s.
        ///
        /// # Note
        ///
        /// One part where this analogy breaks down is that the customer could be directed to
        /// any [`Shop`] in the cluster to place an order, but if he chooses not to wait for
        /// the order to be ready, he will end up picking up the order from a different [`Shop`]
        /// than the one he ordered, and perhaps even a different one to the one that made the
        /// coffee.
        ///
        /// This can possibly be solved by making the Application Load Balancer sticky, so that
        /// the customer is always directed to the same [`Shop`] to pick up the order; but this
        /// is not necessary in practice.
        ///
        /// Perhaps the problem is with the real world - why shouldn't Starbucks be able to
        /// do that?
        pub struct Shop<I, O, F>
        where
            I: Serialize + DeserializeOwned,
            O: Serialize + DeserializeOwned,
            F: Machine<I, O>,
        {
            /// The name of the task that this shop is responsible for.
            ///
            /// This is used to ensure multicast messages are only processed by the correct shop.
            /// Ideally, each shop should use unique multicast addresses to prevent message collisions.
            pub name: String,
            /// A map of tickets to their respective [`Notify`] events that are used to notify the
            /// waiter when a ticket is ready.
            pub tickets: RwLock<HashMap<String, Notify>>,
            /// The coffee machine that will process tickets.
            ///
            /// This is the actual task that will be executed when a ticket is received. It should be able
            /// to tell apart any different types of tickets among the generic input type `I`, and produce
            /// a generic output type `O` regardless of the input type.
            coffee_machine: F,
            /// Dynamodb table name to store the finished products.
            pub dynamodb_table: String,
            /// The configuration for the shop.
            ///
            /// These include the settings for the multicast address, the port, and the IP address, number
            /// of baristas etc.
            pub config: Config,
            /// The AWS SDK configuration for the shop.
            pub aws_config: helpers::aws::SdkConfig,
            /// Phantom data to attach the input and output types to the shop.
            _phantom: PhantomData<(I, O)>,
        }
        #[automatically_derived]
        impl<
            I: ::core::fmt::Debug,
            O: ::core::fmt::Debug,
            F: ::core::fmt::Debug,
        > ::core::fmt::Debug for Shop<I, O, F>
        where
            I: Serialize + DeserializeOwned,
            O: Serialize + DeserializeOwned,
            F: Machine<I, O>,
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "name",
                    "tickets",
                    "coffee_machine",
                    "dynamodb_table",
                    "config",
                    "aws_config",
                    "_phantom",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.name,
                    &self.tickets,
                    &self.coffee_machine,
                    &self.dynamodb_table,
                    &self.config,
                    &self.aws_config,
                    &&self._phantom,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Shop",
                    names,
                    values,
                )
            }
        }
        impl<I, O, F> Shop<I, O, F>
        where
            I: Serialize + DeserializeOwned,
            O: Serialize + DeserializeOwned,
            F: Machine<I, O>,
        {
            /// Create a new shop with the given name, coffee machine, and configuration.
            pub async fn new(
                name: String,
                coffee_machine: F,
                mut config: Config,
                aws_config: Option<helpers::aws::SdkConfig>,
            ) -> Result<Arc<Self>, CoffeeShopError> {
                let dynamodb_table = config
                    .dynamodb_table
                    .take()
                    .unwrap_or_else(|| ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}{1}", DYNAMODB_TABLE_PREFIX, &name),
                        );
                        res
                    }));
                let aws_config = if let Some(aws_config) = aws_config {
                    aws_config
                } else {
                    helpers::aws::get_aws_config().await?
                };
                Ok(
                    Arc::new(Self {
                        name,
                        tickets: HashMap::new().into(),
                        coffee_machine,
                        dynamodb_table,
                        config,
                        aws_config,
                        _phantom: PhantomData,
                    }),
                )
            }
            /// Open the shop, start listening for requests.
            pub async fn open(&self) -> Result<(), CoffeeShopError> {
                helpers::sts::report_aws_login(Some(&self.aws_config)).await?;
                ::core::panicking::panic("not implemented")
            }
        }
    }
    pub use shop::Shop;
    mod machine {
        use std::future::Future;
        /// A trait that defines the behavior of a coffee machine, i.e. the function
        /// that will be called when a ticket is received, and outputs the result
        /// to the DynamoDB table.
        ///
        /// # Note
        ///
        /// Async closures are not expected to be in Stable until at least Q1 2025,
        /// therefore we could not globally implement the `AsyncFn` trait yet for this
        /// trait.
        ///
        /// To use this trait, you must implement the `Machine` trait for your struct
        /// and define the `call` method, which can be a simple wrapper around an async
        /// function.
        pub trait Machine<I, O>: Clone + Send + Sized {
            type Future: Future<Output = O> + Send;
            fn call(self, input: I) -> Self::Future;
        }
    }
    pub use machine::Machine;
    mod proto {
        //! protobuf structs and their implementations.
        //!
        //! The [`MulticastMessage`] struct is the main struct used in this crate. It supports
        //! serialization and deserialization of Generic types to and from bytes, hashing,
        //! and checksum generation.
        //!
        //! This allows a complex type to be serialized into a [`MulticastMessage`] and then
        //! broadcasted to other nodes listening on the same multicast address.
        pub struct MulticastMessage {
            #[prost(string, tag = "99")]
            pub task: ::prost::alloc::string::String,
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
            #[prost(enumeration = "multicast_message::Kind", tag = "2")]
            pub kind: i32,
            #[prost(message, optional, tag = "3")]
            pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
            #[prost(enumeration = "multicast_message::Status", tag = "4")]
            pub status: i32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for MulticastMessage {
            #[inline]
            fn clone(&self) -> MulticastMessage {
                MulticastMessage {
                    task: ::core::clone::Clone::clone(&self.task),
                    id: ::core::clone::Clone::clone(&self.id),
                    kind: ::core::clone::Clone::clone(&self.kind),
                    timestamp: ::core::clone::Clone::clone(&self.timestamp),
                    status: ::core::clone::Clone::clone(&self.status),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for MulticastMessage {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for MulticastMessage {
            #[inline]
            fn eq(&self, other: &MulticastMessage) -> bool {
                self.task == other.task && self.id == other.id && self.kind == other.kind
                    && self.timestamp == other.timestamp && self.status == other.status
            }
        }
        impl ::prost::Message for MulticastMessage {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if self.id != "" {
                    ::prost::encoding::string::encode(1u32, &self.id, buf);
                }
                if self.kind != multicast_message::Kind::default() as i32 {
                    ::prost::encoding::int32::encode(2u32, &self.kind, buf);
                }
                if let Some(ref msg) = self.timestamp {
                    ::prost::encoding::message::encode(3u32, msg, buf);
                }
                if self.status != multicast_message::Status::default() as i32 {
                    ::prost::encoding::int32::encode(4u32, &self.status, buf);
                }
                if self.task != "" {
                    ::prost::encoding::string::encode(99u32, &self.task, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "MulticastMessage";
                match tag {
                    1u32 => {
                        let mut value = &mut self.id;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "id");
                                error
                            })
                    }
                    2u32 => {
                        let mut value = &mut self.kind;
                        ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "kind");
                                error
                            })
                    }
                    3u32 => {
                        let mut value = &mut self.timestamp;
                        ::prost::encoding::message::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "timestamp");
                                error
                            })
                    }
                    4u32 => {
                        let mut value = &mut self.status;
                        ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "status");
                                error
                            })
                    }
                    99u32 => {
                        let mut value = &mut self.task;
                        ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "task");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + if self.id != "" {
                        ::prost::encoding::string::encoded_len(1u32, &self.id)
                    } else {
                        0
                    }
                    + if self.kind != multicast_message::Kind::default() as i32 {
                        ::prost::encoding::int32::encoded_len(2u32, &self.kind)
                    } else {
                        0
                    }
                    + self
                        .timestamp
                        .as_ref()
                        .map_or(
                            0,
                            |msg| ::prost::encoding::message::encoded_len(3u32, msg),
                        )
                    + if self.status != multicast_message::Status::default() as i32 {
                        ::prost::encoding::int32::encoded_len(4u32, &self.status)
                    } else {
                        0
                    }
                    + if self.task != "" {
                        ::prost::encoding::string::encoded_len(99u32, &self.task)
                    } else {
                        0
                    }
            }
            fn clear(&mut self) {
                self.id.clear();
                self.kind = multicast_message::Kind::default() as i32;
                self.timestamp = ::core::option::Option::None;
                self.status = multicast_message::Status::default() as i32;
                self.task.clear();
            }
        }
        impl ::core::default::Default for MulticastMessage {
            fn default() -> Self {
                MulticastMessage {
                    id: ::prost::alloc::string::String::new(),
                    kind: multicast_message::Kind::default() as i32,
                    timestamp: ::core::default::Default::default(),
                    status: multicast_message::Status::default() as i32,
                    task: ::prost::alloc::string::String::new(),
                }
            }
        }
        impl ::core::fmt::Debug for MulticastMessage {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("MulticastMessage");
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.task)
                    };
                    builder.field("task", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        #[allow(non_snake_case)]
                        fn ScalarWrapper<T>(v: T) -> T {
                            v
                        }
                        ScalarWrapper(&self.id)
                    };
                    builder.field("id", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let res: ::core::result::Result<
                                    multicast_message::Kind,
                                    _,
                                > = ::core::convert::TryFrom::try_from(*self.0);
                                match res {
                                    Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                    Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ScalarWrapper(&self.kind)
                    };
                    builder.field("kind", &wrapper)
                };
                let builder = {
                    let wrapper = &self.timestamp;
                    builder.field("timestamp", &wrapper)
                };
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a i32);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                let res: ::core::result::Result<
                                    multicast_message::Status,
                                    _,
                                > = ::core::convert::TryFrom::try_from(*self.0);
                                match res {
                                    Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                    Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                }
                            }
                        }
                        ScalarWrapper(&self.status)
                    };
                    builder.field("status", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl MulticastMessage {
            ///Returns the enum value of `kind`, or the default if the field is set to an invalid enum value.
            pub fn kind(&self) -> multicast_message::Kind {
                ::core::convert::TryFrom::try_from(self.kind)
                    .unwrap_or(multicast_message::Kind::default())
            }
            ///Sets `kind` to the provided enum value.
            pub fn set_kind(&mut self, value: multicast_message::Kind) {
                self.kind = value as i32;
            }
            ///Returns the enum value of `status`, or the default if the field is set to an invalid enum value.
            pub fn status(&self) -> multicast_message::Status {
                ::core::convert::TryFrom::try_from(self.status)
                    .unwrap_or(multicast_message::Status::default())
            }
            ///Sets `status` to the provided enum value.
            pub fn set_status(&mut self, value: multicast_message::Status) {
                self.status = value as i32;
            }
        }
        /// Nested message and enum types in `MulticastMessage`.
        pub mod multicast_message {
            #[repr(i32)]
            pub enum Kind {
                Ticket = 0,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Kind {
                #[inline]
                fn clone(&self) -> Kind {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Kind {}
            #[automatically_derived]
            impl ::core::fmt::Debug for Kind {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Ticket")
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Kind {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Kind {
                #[inline]
                fn eq(&self, other: &Kind) -> bool {
                    true
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for Kind {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::hash::Hash for Kind {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {}
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for Kind {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &Kind,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for Kind {
                #[inline]
                fn cmp(&self, other: &Kind) -> ::core::cmp::Ordering {
                    ::core::cmp::Ordering::Equal
                }
            }
            impl Kind {
                ///Returns `true` if `value` is a variant of `Kind`.
                pub fn is_valid(value: i32) -> bool {
                    match value {
                        0 => true,
                        _ => false,
                    }
                }
                #[deprecated = "Use the TryFrom<i32> implementation instead"]
                ///Converts an `i32` to a `Kind`, or `None` if `value` is not a valid variant.
                pub fn from_i32(value: i32) -> ::core::option::Option<Kind> {
                    match value {
                        0 => ::core::option::Option::Some(Kind::Ticket),
                        _ => ::core::option::Option::None,
                    }
                }
            }
            impl ::core::default::Default for Kind {
                fn default() -> Kind {
                    Kind::Ticket
                }
            }
            impl ::core::convert::From<Kind> for i32 {
                fn from(value: Kind) -> i32 {
                    value as i32
                }
            }
            impl ::core::convert::TryFrom<i32> for Kind {
                type Error = ::prost::UnknownEnumValue;
                fn try_from(
                    value: i32,
                ) -> ::core::result::Result<Kind, ::prost::UnknownEnumValue> {
                    match value {
                        0 => ::core::result::Result::Ok(Kind::Ticket),
                        _ => {
                            ::core::result::Result::Err(::prost::UnknownEnumValue(value))
                        }
                    }
                }
            }
            impl Kind {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Self::Ticket => "TICKET",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "TICKET" => Some(Self::Ticket),
                        _ => None,
                    }
                }
            }
            #[repr(i32)]
            pub enum Status {
                Rejected = 0,
                Complete = 1,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Status {
                #[inline]
                fn clone(&self) -> Status {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Status {}
            #[automatically_derived]
            impl ::core::fmt::Debug for Status {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Status::Rejected => "Rejected",
                            Status::Complete => "Complete",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Status {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Status {
                #[inline]
                fn eq(&self, other: &Status) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for Status {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::hash::Hash for Status {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_discr, state)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::PartialOrd for Status {
                #[inline]
                fn partial_cmp(
                    &self,
                    other: &Status,
                ) -> ::core::option::Option<::core::cmp::Ordering> {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Ord for Status {
                #[inline]
                fn cmp(&self, other: &Status) -> ::core::cmp::Ordering {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
                }
            }
            impl Status {
                ///Returns `true` if `value` is a variant of `Status`.
                pub fn is_valid(value: i32) -> bool {
                    match value {
                        0 => true,
                        1 => true,
                        _ => false,
                    }
                }
                #[deprecated = "Use the TryFrom<i32> implementation instead"]
                ///Converts an `i32` to a `Status`, or `None` if `value` is not a valid variant.
                pub fn from_i32(value: i32) -> ::core::option::Option<Status> {
                    match value {
                        0 => ::core::option::Option::Some(Status::Rejected),
                        1 => ::core::option::Option::Some(Status::Complete),
                        _ => ::core::option::Option::None,
                    }
                }
            }
            impl ::core::default::Default for Status {
                fn default() -> Status {
                    Status::Rejected
                }
            }
            impl ::core::convert::From<Status> for i32 {
                fn from(value: Status) -> i32 {
                    value as i32
                }
            }
            impl ::core::convert::TryFrom<i32> for Status {
                type Error = ::prost::UnknownEnumValue;
                fn try_from(
                    value: i32,
                ) -> ::core::result::Result<Status, ::prost::UnknownEnumValue> {
                    match value {
                        0 => ::core::result::Result::Ok(Status::Rejected),
                        1 => ::core::result::Result::Ok(Status::Complete),
                        _ => {
                            ::core::result::Result::Err(::prost::UnknownEnumValue(value))
                        }
                    }
                }
            }
            impl Status {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Self::Rejected => "REJECTED",
                        Self::Complete => "COMPLETE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "REJECTED" => Some(Self::Rejected),
                        "COMPLETE" => Some(Self::Complete),
                        _ => None,
                    }
                }
            }
        }
        pub use multicast_message::{
            Kind as MulticastMessageKind, Status as MulticastMessageStatus,
        };
        mod implementations {
            //! Additional implementations for the protobuf generated structs.
            use super::{MulticastMessage, MulticastMessageKind, MulticastMessageStatus};
            mod new {
                use super::{
                    MulticastMessage, MulticastMessageKind, MulticastMessageStatus,
                };
                impl MulticastMessage {
                    /// Creates a new `MulticastMessage` with the given `id` and `kind`.
                    pub fn new(
                        task: &str,
                        id: &str,
                        kind: MulticastMessageKind,
                        status: MulticastMessageStatus,
                    ) -> Self {
                        Self {
                            task: task.to_owned(),
                            id: id.to_owned(),
                            kind: kind.into(),
                            timestamp: Some(
                                prost_types::Timestamp::from(std::time::SystemTime::now()),
                            ),
                            status: status.into(),
                        }
                    }
                    /// Creates a new `MulticastMessage` with the given `id` and `kind` set to `Ticket`,
                    /// and `status` set to `Complete`.
                    pub fn new_ticket_complete(task: &str, id: &str) -> Self {
                        Self::new(
                            task,
                            id,
                            MulticastMessageKind::Ticket,
                            MulticastMessageStatus::Complete,
                        )
                    }
                    /// Creates a new `MulticastMessage` with the given `id` and `kind` set to `Ticket`,
                    /// and `status` set to `Rejected`.
                    pub fn new_ticket_rejected(task: &str, id: &str) -> Self {
                        Self::new(
                            task,
                            id,
                            MulticastMessageKind::Ticket,
                            MulticastMessageStatus::Rejected,
                        )
                    }
                }
            }
        }
    }
    pub use proto::MulticastMessage;
    mod waiter {
        //! A waiter is an async HTTP host that listens for incoming requests and insert them into
        //! the specified AWS SQS queue.
        //! For synchronous requests, the waiter will also asynchronously await a [`Notify`](tokio::sync::Notify)
        //! event from the multicast channel and report back to the client when the request had been processed.
        #![allow(unused_variables)]
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };
        use tokio::sync::Notify;
        use axum::extract::{Json, Query};
        use axum::{
            http::{header, StatusCode},
            response::IntoResponse,
        };
        use super::{
            message::{self, QueryType},
            Machine, Shop, Ticket,
        };
        use crate::CoffeeShopError;
        /// A [`Waiter`] instance that acts as an async REST API host.
        pub struct Waiter<Q, I, O, F>
        where
            Q: message::QueryType,
            I: serde::de::DeserializeOwned + serde::Serialize,
            O: serde::Serialize + serde::de::DeserializeOwned,
            F: Machine<I, O>,
        {
            /// The back reference to the shop that this waiter is serving.
            pub shop: Arc<Shop<I, O, F>>,
            /// The total amount of historical requests processed.
            /// Only the [`request`](Self::request) and [`async_request`](Self::async_request) methods
            /// will increment this counter.
            ///
            /// Internally, this is done by [`create_ticket`](Self::create_ticket).
            pub request_count: Arc<AtomicUsize>,
            pub start_time: tokio::time::Instant,
            _phantom: std::marker::PhantomData<(Q, I, O)>,
        }
        #[automatically_derived]
        impl<
            Q: ::core::fmt::Debug,
            I: ::core::fmt::Debug,
            O: ::core::fmt::Debug,
            F: ::core::fmt::Debug,
        > ::core::fmt::Debug for Waiter<Q, I, O, F>
        where
            Q: message::QueryType,
            I: serde::de::DeserializeOwned + serde::Serialize,
            O: serde::Serialize + serde::de::DeserializeOwned,
            F: Machine<I, O>,
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Waiter",
                    "shop",
                    &self.shop,
                    "request_count",
                    &self.request_count,
                    "start_time",
                    &self.start_time,
                    "_phantom",
                    &&self._phantom,
                )
            }
        }
        impl<Q, I, O, F> Waiter<Q, I, O, F>
        where
            Q: message::QueryType,
            I: serde::de::DeserializeOwned + serde::Serialize,
            O: serde::Serialize + serde::de::DeserializeOwned,
            F: Machine<I, O>,
        {
            /// `GET` Handler for getting the status of the waiter.
            pub async fn status(&self) -> impl IntoResponse {
                (
                    StatusCode::OK,
                    [
                        (header::CONTENT_TYPE, "application/json"),
                        (header::CACHE_CONTROL, "no-store"),
                    ],
                    Json(message::StatusResponse {
                        metadata: message::ResponseMetadata::new(&self.start_time),
                        request_count: self.request_count.load(Ordering::Relaxed),
                        ticket_count: self.shop.tickets.read().await.len(),
                    }),
                )
            }
            /// `POST` Handler for incoming requests.
            pub async fn request(
                &self,
                Query(params): Query<Q>,
                Json(payload): Json<I>,
            ) -> impl IntoResponse {
                let timeout = params.get_timeout();
                self.create_and_retrieve_ticket(
                        message::CombinedInput {
                            query: params,
                            input: Some(payload),
                        },
                        timeout,
                    )
                    .await
                    .map(|(ticket, output)| message::OutputResponse {
                        ticket,
                        metadata: message::ResponseMetadata::new(&self.start_time),
                        output,
                    })
            }
            /// `POST` Handler for asynchronous requests.
            ///
            /// This immediately returns a `202 Accepted` response with
            /// the ticket ID as the body.
            pub async fn async_request(
                &self,
                Query(params): Query<Q>,
                Json(payload): Json<I>,
            ) -> impl IntoResponse {
                self.create_ticket(message::CombinedInput {
                        query: params,
                        input: Some(payload),
                    })
                    .await
                    .map(|(ticket, _)| message::TicketResponse {
                        ticket,
                        metadata: message::ResponseMetadata::new(&self.start_time),
                    })
            }
            /// A `GET` request to fetch results from a previously processed request.
            pub async fn async_retrieve(
                &self,
                Query(params): Query<message::TicketQuery>,
            ) -> impl IntoResponse {
                let timeout = params.get_timeout();
                self.retrieve_ticket_timeout(params.ticket, timeout)
                    .await
                    .map(|(ticket, output)| message::OutputResponse {
                        ticket,
                        metadata: message::ResponseMetadata::new(&self.start_time),
                        output,
                    })
            }
            /// An internal method to create a new ticket on the AWS SQS queue,
            /// then return the [`Notify`] instance to await the result.
            async fn create_ticket(
                &self,
                input: message::CombinedInput<Q, I>,
            ) -> Result<(message::Ticket, Arc<Notify>), CoffeeShopError> {
                self.request_count.fetch_add(1, Ordering::Relaxed);
                ::core::panicking::panic("not yet implemented")
            }
            /// An internal method to retrieve the result of a ticket from the
            /// AWS SQS queue.
            async fn retrieve_ticket(
                &self,
                ticket: String,
            ) -> Result<(Ticket, O), CoffeeShopError> {
                ::core::panicking::panic("not yet implemented")
            }
            /// An internal method to retrieve the result of a ticket with a timeout;
            /// if the timeout is reached, an [`CoffeeShopError::RetrieveTimeout`] is returned.
            async fn retrieve_ticket_timeout(
                &self,
                ticket: String,
                timeout: Option<tokio::time::Duration>,
            ) -> Result<(Ticket, O), CoffeeShopError> {
                if let Some(timeout) = timeout {
                    {
                        #[doc(hidden)]
                        mod __tokio_select_util {
                            pub(super) enum Out<_0, _1> {
                                _0(_0),
                                _1(_1),
                                Disabled,
                            }
                            pub(super) type Mask = u8;
                        }
                        use ::tokio::macros::support::Future;
                        use ::tokio::macros::support::Pin;
                        use ::tokio::macros::support::Poll::{Ready, Pending};
                        const BRANCHES: u32 = 2;
                        let mut disabled: __tokio_select_util::Mask = Default::default();
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 0;
                            disabled |= mask;
                        }
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 1;
                            disabled |= mask;
                        }
                        let mut output = {
                            let futures_init = (
                                tokio::time::sleep(timeout),
                                self.retrieve_ticket(ticket),
                            );
                            let mut futures = (
                                ::tokio::macros::support::IntoFuture::into_future(
                                    futures_init.0,
                                ),
                                ::tokio::macros::support::IntoFuture::into_future(
                                    futures_init.1,
                                ),
                            );
                            let mut futures = &mut futures;
                            ::tokio::macros::support::poll_fn(|cx| {
                                    let mut is_pending = false;
                                    let start = {
                                        ::tokio::macros::support::thread_rng_n(BRANCHES)
                                    };
                                    for i in 0..BRANCHES {
                                        let branch;
                                        #[allow(clippy::modulo_one)]
                                        {
                                            branch = (start + i) % BRANCHES;
                                        }
                                        match branch {
                                            #[allow(unreachable_code)]
                                            0 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    _ => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_0(out));
                                            }
                                            #[allow(unreachable_code)]
                                            1 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (_, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    result => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_1(out));
                                            }
                                            _ => {
                                                ::core::panicking::panic_fmt(
                                                    format_args!(
                                                        "internal error: entered unreachable code: {0}",
                                                        format_args!(
                                                            "reaching this means there probably is an off by one bug",
                                                        ),
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    if is_pending {
                                        Pending
                                    } else {
                                        Ready(__tokio_select_util::Out::Disabled)
                                    }
                                })
                                .await
                        };
                        match output {
                            __tokio_select_util::Out::_0(_) => {
                                Err(CoffeeShopError::RetrieveTimeout(timeout))
                            }
                            __tokio_select_util::Out::_1(result) => result,
                            __tokio_select_util::Out::Disabled => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "all branches are disabled and there is no else branch",
                                    ),
                                );
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("failed to match bind"),
                                    ),
                                );
                            }
                        }
                    }
                } else {
                    self.retrieve_ticket(ticket).await
                }
            }
            /// An internal method to create a new ticket, wait for the result,
            /// then return the result to the client.
            async fn create_and_retrieve_ticket(
                &self,
                input: message::CombinedInput<Q, I>,
                timeout: Option<tokio::time::Duration>,
            ) -> Result<(Ticket, O), CoffeeShopError> {
                let (ticket, notify) = self.create_ticket(input).await?;
                if let Some(timeout) = timeout {
                    {
                        #[doc(hidden)]
                        mod __tokio_select_util {
                            pub(super) enum Out<_0, _1> {
                                _0(_0),
                                _1(_1),
                                Disabled,
                            }
                            pub(super) type Mask = u8;
                        }
                        use ::tokio::macros::support::Future;
                        use ::tokio::macros::support::Pin;
                        use ::tokio::macros::support::Poll::{Ready, Pending};
                        const BRANCHES: u32 = 2;
                        let mut disabled: __tokio_select_util::Mask = Default::default();
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 0;
                            disabled |= mask;
                        }
                        if !true {
                            let mask: __tokio_select_util::Mask = 1 << 1;
                            disabled |= mask;
                        }
                        let mut output = {
                            let futures_init = (
                                notify.notified(),
                                tokio::time::sleep(timeout),
                            );
                            let mut futures = (
                                ::tokio::macros::support::IntoFuture::into_future(
                                    futures_init.0,
                                ),
                                ::tokio::macros::support::IntoFuture::into_future(
                                    futures_init.1,
                                ),
                            );
                            let mut futures = &mut futures;
                            ::tokio::macros::support::poll_fn(|cx| {
                                    let mut is_pending = false;
                                    let start = {
                                        ::tokio::macros::support::thread_rng_n(BRANCHES)
                                    };
                                    for i in 0..BRANCHES {
                                        let branch;
                                        #[allow(clippy::modulo_one)]
                                        {
                                            branch = (start + i) % BRANCHES;
                                        }
                                        match branch {
                                            #[allow(unreachable_code)]
                                            0 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    _ => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_0(out));
                                            }
                                            #[allow(unreachable_code)]
                                            1 => {
                                                let mask = 1 << branch;
                                                if disabled & mask == mask {
                                                    continue;
                                                }
                                                let (_, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                let out = match Future::poll(fut, cx) {
                                                    Ready(out) => out,
                                                    Pending => {
                                                        is_pending = true;
                                                        continue;
                                                    }
                                                };
                                                disabled |= mask;
                                                #[allow(unused_variables)] #[allow(unused_mut)]
                                                match &out {
                                                    _ => {}
                                                    _ => continue,
                                                }
                                                return Ready(__tokio_select_util::Out::_1(out));
                                            }
                                            _ => {
                                                ::core::panicking::panic_fmt(
                                                    format_args!(
                                                        "internal error: entered unreachable code: {0}",
                                                        format_args!(
                                                            "reaching this means there probably is an off by one bug",
                                                        ),
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                    if is_pending {
                                        Pending
                                    } else {
                                        Ready(__tokio_select_util::Out::Disabled)
                                    }
                                })
                                .await
                        };
                        match output {
                            __tokio_select_util::Out::_0(_) => {
                                self.retrieve_ticket(ticket).await
                            }
                            __tokio_select_util::Out::_1(_) => {
                                Err(CoffeeShopError::RetrieveTimeout(timeout))
                            }
                            __tokio_select_util::Out::Disabled => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "all branches are disabled and there is no else branch",
                                    ),
                                );
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("failed to match bind"),
                                    ),
                                );
                            }
                        }
                    }
                } else {
                    notify.notified().await;
                    self.retrieve_ticket(ticket).await
                }
            }
        }
    }
    pub use waiter::*;
    pub mod message {
        //! This module contains the internal data structures for messaging between
        //! structs.
        mod input {
            use std::collections::BTreeMap;
            use super::QueryType;
            pub struct Q {}
            #[automatically_derived]
            impl ::core::fmt::Debug for Q {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "Q")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Q {
                #[inline]
                fn clone(&self) -> Q {
                    Q {}
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for Q {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Q",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for Q {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<Q>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = Q;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Q",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(Q {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(Q {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Q",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Q>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            pub struct I {}
            #[automatically_derived]
            impl ::core::fmt::Debug for I {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "I")
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for I {
                #[inline]
                fn clone(&self) -> I {
                    I {}
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for I {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "I",
                            false as usize,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for I {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<I>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = I;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct I",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                _: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                _serde::__private::Ok(I {})
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                _serde::__private::Ok(I {})
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "I",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<I>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            pub struct MockInput {
                pub query: Q,
                pub input: Option<I>,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for MockInput {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "query" => _serde::__private::Ok(__Field::__field0),
                                    "input" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"query" => _serde::__private::Ok(__Field::__field0),
                                    b"input" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<MockInput>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = MockInput;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct MockInput",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    Q,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct MockInput with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<I>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct MockInput with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(MockInput {
                                    query: __field0,
                                    input: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Q> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<Option<I>> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("query"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Q>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("input"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Option<I>>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("query")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("input")?
                                    }
                                };
                                _serde::__private::Ok(MockInput {
                                    query: __field0,
                                    input: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["query", "input"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "MockInput",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<MockInput>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            /// A struct that combines a query and an input into a single struct.
            ///
            /// This is for the purpose of passing a complete set of HTTP request data to the handler,
            /// allowing the freedom to design a REST structure that fits the application's needs.
            pub struct CombinedInput<Q, I>
            where
                Q: QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                pub query: Q,
                pub input: Option<I>,
            }
            #[automatically_derived]
            impl<Q: ::core::fmt::Debug, I: ::core::fmt::Debug> ::core::fmt::Debug
            for CombinedInput<Q, I>
            where
                Q: QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "CombinedInput",
                        "query",
                        &self.query,
                        "input",
                        &&self.input,
                    )
                }
            }
            #[automatically_derived]
            impl<Q: ::core::clone::Clone, I: ::core::clone::Clone> ::core::clone::Clone
            for CombinedInput<Q, I>
            where
                Q: QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                #[inline]
                fn clone(&self) -> CombinedInput<Q, I> {
                    CombinedInput {
                        query: ::core::clone::Clone::clone(&self.query),
                        input: ::core::clone::Clone::clone(&self.input),
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<Q, I> _serde::Serialize for CombinedInput<Q, I>
                where
                    Q: QueryType,
                    I: serde::de::DeserializeOwned + serde::Serialize,
                    Q: _serde::Serialize,
                    I: _serde::Serialize,
                {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CombinedInput",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "query",
                            &self.query,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "input",
                            &self.input,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
        }
        pub use input::*;
        mod metadata {
            use chrono::{DateTime, Utc};
            use gethostname::gethostname as get_hostname;
            use std::ffi::OsString;
            use tokio::time::Duration;
            /// Response Metadata, containing information about the host returning the response.
            ///
            /// Mostly for debugging purposes.
            pub struct ResponseMetadata {
                /// The IP address of the server.
                pub hostname: OsString,
                /// The timestamp of the response.
                pub timestamp: DateTime<Utc>,
                /// Server uptime in seconds.
                pub uptime: Duration,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ResponseMetadata {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ResponseMetadata",
                        "hostname",
                        &self.hostname,
                        "timestamp",
                        &self.timestamp,
                        "uptime",
                        &&self.uptime,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for ResponseMetadata {
                #[inline]
                fn clone(&self) -> ResponseMetadata {
                    ResponseMetadata {
                        hostname: ::core::clone::Clone::clone(&self.hostname),
                        timestamp: ::core::clone::Clone::clone(&self.timestamp),
                        uptime: ::core::clone::Clone::clone(&self.uptime),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ResponseMetadata {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ResponseMetadata {
                #[inline]
                fn eq(&self, other: &ResponseMetadata) -> bool {
                    self.hostname == other.hostname && self.timestamp == other.timestamp
                        && self.uptime == other.uptime
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for ResponseMetadata {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<OsString>;
                    let _: ::core::cmp::AssertParamIsEq<DateTime<Utc>>;
                    let _: ::core::cmp::AssertParamIsEq<Duration>;
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for ResponseMetadata {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "ResponseMetadata",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "hostname",
                            &self.hostname,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "timestamp",
                            &self.timestamp,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "uptime",
                            &self.uptime,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for ResponseMetadata {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "hostname" => _serde::__private::Ok(__Field::__field0),
                                    "timestamp" => _serde::__private::Ok(__Field::__field1),
                                    "uptime" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"hostname" => _serde::__private::Ok(__Field::__field0),
                                    b"timestamp" => _serde::__private::Ok(__Field::__field1),
                                    b"uptime" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<ResponseMetadata>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = ResponseMetadata;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct ResponseMetadata",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    OsString,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct ResponseMetadata with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    DateTime<Utc>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct ResponseMetadata with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Duration,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct ResponseMetadata with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(ResponseMetadata {
                                    hostname: __field0,
                                    timestamp: __field1,
                                    uptime: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<OsString> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    DateTime<Utc>,
                                > = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<Duration> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "hostname",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<OsString>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "timestamp",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    DateTime<Utc>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("uptime"),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Duration>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("hostname")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("timestamp")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("uptime")?
                                    }
                                };
                                _serde::__private::Ok(ResponseMetadata {
                                    hostname: __field0,
                                    timestamp: __field1,
                                    uptime: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "hostname",
                            "timestamp",
                            "uptime",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "ResponseMetadata",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<ResponseMetadata>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            impl ResponseMetadata {
                /// Create a new [`ResponseMetadata`] instance.
                pub fn new(start_time: &tokio::time::Instant) -> Self {
                    Self {
                        hostname: get_hostname(),
                        timestamp: Utc::now(),
                        uptime: start_time.elapsed(),
                    }
                }
            }
        }
        pub use metadata::*;
        mod query {
            /// [`QueryType`] is a trait that defines the methods that a query type must implement.
            ///
            /// This allows the designer to customise the query parameters to their needs, while
            /// maintaining a standardised interface for the waiter to know certain information about
            /// the query.
            pub trait QueryType: serde::de::DeserializeOwned + serde::Serialize {
                /// Get the timeout for the query.
                ///
                /// This is used to determine how long the waiter should wait for a response
                /// before issuing a [`http::StatusCode::REQUEST_TIMEOUT`] response.
                ///
                /// While a [`None`] value is allowed, it is strongly recommended to enforce a
                /// [`Some<Duration>`] value to prevent the waiter from waiting indefinitely.
                fn get_timeout(&self) -> Option<tokio::time::Duration>;
            }
        }
        pub use query::*;
        mod status {
            use super::ResponseMetadata;
            /// Status report of the waiter.
            pub struct StatusResponse {
                /// Metadata of the response.
                pub metadata: ResponseMetadata,
                /// dequest count.
                pub request_count: usize,
                /// Ticket count.
                pub ticket_count: usize,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for StatusResponse {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "StatusResponse",
                        "metadata",
                        &self.metadata,
                        "request_count",
                        &self.request_count,
                        "ticket_count",
                        &&self.ticket_count,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for StatusResponse {
                #[inline]
                fn clone(&self) -> StatusResponse {
                    StatusResponse {
                        metadata: ::core::clone::Clone::clone(&self.metadata),
                        request_count: ::core::clone::Clone::clone(&self.request_count),
                        ticket_count: ::core::clone::Clone::clone(&self.ticket_count),
                    }
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for StatusResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "StatusResponse",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "metadata",
                            &self.metadata,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "request_count",
                            &self.request_count,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "ticket_count",
                            &self.ticket_count,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for StatusResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "metadata" => _serde::__private::Ok(__Field::__field0),
                                    "request_count" => _serde::__private::Ok(__Field::__field1),
                                    "ticket_count" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"metadata" => _serde::__private::Ok(__Field::__field0),
                                    b"request_count" => _serde::__private::Ok(__Field::__field1),
                                    b"ticket_count" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<StatusResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = StatusResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct StatusResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    ResponseMetadata,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct StatusResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    usize,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct StatusResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    usize,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct StatusResponse with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(StatusResponse {
                                    metadata: __field0,
                                    request_count: __field1,
                                    ticket_count: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    ResponseMetadata,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<usize> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<usize> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "metadata",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    ResponseMetadata,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "request_count",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "ticket_count",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("metadata")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("request_count")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("ticket_count")?
                                    }
                                };
                                _serde::__private::Ok(StatusResponse {
                                    metadata: __field0,
                                    request_count: __field1,
                                    ticket_count: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "metadata",
                            "request_count",
                            "ticket_count",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "StatusResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<StatusResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
        }
        pub use status::*;
        mod response {
            use axum::{body::Body, http, response::IntoResponse, Json};
            use super::{ResponseMetadata, Ticket};
            /// Response message for the output of a request.
            pub struct OutputResponse<O>
            where
                O: serde::Serialize,
            {
                pub ticket: Ticket,
                pub metadata: ResponseMetadata,
                pub output: O,
            }
            #[automatically_derived]
            impl<O: ::core::fmt::Debug> ::core::fmt::Debug for OutputResponse<O>
            where
                O: serde::Serialize,
            {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "OutputResponse",
                        "ticket",
                        &self.ticket,
                        "metadata",
                        &self.metadata,
                        "output",
                        &&self.output,
                    )
                }
            }
            #[automatically_derived]
            impl<O: ::core::clone::Clone> ::core::clone::Clone for OutputResponse<O>
            where
                O: serde::Serialize,
            {
                #[inline]
                fn clone(&self) -> OutputResponse<O> {
                    OutputResponse {
                        ticket: ::core::clone::Clone::clone(&self.ticket),
                        metadata: ::core::clone::Clone::clone(&self.metadata),
                        output: ::core::clone::Clone::clone(&self.output),
                    }
                }
            }
            #[automatically_derived]
            impl<O> ::core::marker::StructuralPartialEq for OutputResponse<O>
            where
                O: serde::Serialize,
            {}
            #[automatically_derived]
            impl<O: ::core::cmp::PartialEq> ::core::cmp::PartialEq for OutputResponse<O>
            where
                O: serde::Serialize,
            {
                #[inline]
                fn eq(&self, other: &OutputResponse<O>) -> bool {
                    self.ticket == other.ticket && self.metadata == other.metadata
                        && self.output == other.output
                }
            }
            #[automatically_derived]
            impl<O: ::core::cmp::Eq> ::core::cmp::Eq for OutputResponse<O>
            where
                O: serde::Serialize,
            {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Ticket>;
                    let _: ::core::cmp::AssertParamIsEq<ResponseMetadata>;
                    let _: ::core::cmp::AssertParamIsEq<O>;
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<O> _serde::Serialize for OutputResponse<O>
                where
                    O: serde::Serialize,
                    O: _serde::Serialize,
                {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "OutputResponse",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "ticket",
                            &self.ticket,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "metadata",
                            &self.metadata,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "output",
                            &self.output,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl<O> IntoResponse for OutputResponse<O>
            where
                O: serde::Serialize,
            {
                fn into_response(self) -> axum::response::Response<Body> {
                    (
                        http::StatusCode::OK,
                        [(axum::http::header::CONTENT_TYPE, "application/json")],
                        Json(self),
                    )
                        .into_response()
                }
            }
        }
        pub use response::*;
        mod ticket {
            use super::{QueryType, ResponseMetadata};
            use axum::response::IntoResponse;
            use serde::{Deserialize, Serialize};
            /// A ticket is a unique identifier for a request that is processed asynchronously.
            ///
            /// This contains the AWS SQS message ID, and the type must be a string.
            pub type Ticket = String;
            /// A query structure to retrieve the result of a ticket.
            pub struct TicketQuery {
                pub ticket: Ticket,
                #[serde_as(r#as = "Option<serde_with::DurationSecondsWithFrac<f64>>")]
                #[serde(default)]
                #[serde(
                    with = ":: serde_with :: As :: < Option < serde_with :: DurationSecondsWithFrac < f64\n> > >"
                )]
                pub timeout: Option<tokio::time::Duration>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for TicketQuery {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "TicketQuery",
                        "ticket",
                        &self.ticket,
                        "timeout",
                        &&self.timeout,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for TicketQuery {
                #[inline]
                fn clone(&self) -> TicketQuery {
                    TicketQuery {
                        ticket: ::core::clone::Clone::clone(&self.ticket),
                        timeout: ::core::clone::Clone::clone(&self.timeout),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for TicketQuery {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for TicketQuery {
                #[inline]
                fn eq(&self, other: &TicketQuery) -> bool {
                    self.ticket == other.ticket && self.timeout == other.timeout
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for TicketQuery {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Ticket>;
                    let _: ::core::cmp::AssertParamIsEq<Option<tokio::time::Duration>>;
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for TicketQuery {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "ticket" => _serde::__private::Ok(__Field::__field0),
                                    "timeout" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"ticket" => _serde::__private::Ok(__Field::__field0),
                                    b"timeout" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<TicketQuery>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = TicketQuery;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct TicketQuery",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    Ticket,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct TicketQuery with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match {
                                    #[doc(hidden)]
                                    struct __DeserializeWith<'de> {
                                        value: Option<tokio::time::Duration>,
                                        phantom: _serde::__private::PhantomData<TicketQuery>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de> _serde::Deserialize<'de>
                                    for __DeserializeWith<'de> {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::__private::Ok(__DeserializeWith {
                                                value: ::serde_with::As::<
                                                    Option<serde_with::DurationSecondsWithFrac<f64>>,
                                                >::deserialize(__deserializer)?,
                                                phantom: _serde::__private::PhantomData,
                                                lifetime: _serde::__private::PhantomData,
                                            })
                                        }
                                    }
                                    _serde::__private::Option::map(
                                        _serde::de::SeqAccess::next_element::<
                                            __DeserializeWith<'de>,
                                        >(&mut __seq)?,
                                        |__wrap| __wrap.value,
                                    )
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        _serde::__private::Default::default()
                                    }
                                };
                                _serde::__private::Ok(TicketQuery {
                                    ticket: __field0,
                                    timeout: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Ticket> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Option<tokio::time::Duration>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("ticket"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Ticket>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "timeout",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some({
                                                #[doc(hidden)]
                                                struct __DeserializeWith<'de> {
                                                    value: Option<tokio::time::Duration>,
                                                    phantom: _serde::__private::PhantomData<TicketQuery>,
                                                    lifetime: _serde::__private::PhantomData<&'de ()>,
                                                }
                                                impl<'de> _serde::Deserialize<'de>
                                                for __DeserializeWith<'de> {
                                                    fn deserialize<__D>(
                                                        __deserializer: __D,
                                                    ) -> _serde::__private::Result<Self, __D::Error>
                                                    where
                                                        __D: _serde::Deserializer<'de>,
                                                    {
                                                        _serde::__private::Ok(__DeserializeWith {
                                                            value: ::serde_with::As::<
                                                                Option<serde_with::DurationSecondsWithFrac<f64>>,
                                                            >::deserialize(__deserializer)?,
                                                            phantom: _serde::__private::PhantomData,
                                                            lifetime: _serde::__private::PhantomData,
                                                        })
                                                    }
                                                }
                                                match _serde::de::MapAccess::next_value::<
                                                    __DeserializeWith<'de>,
                                                >(&mut __map) {
                                                    _serde::__private::Ok(__wrapper) => __wrapper.value,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                }
                                            });
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("ticket")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::Default::default()
                                    }
                                };
                                _serde::__private::Ok(TicketQuery {
                                    ticket: __field0,
                                    timeout: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["ticket", "timeout"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "TicketQuery",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<TicketQuery>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for TicketQuery {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "TicketQuery",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "ticket",
                            &self.ticket,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "timeout",
                            {
                                #[doc(hidden)]
                                struct __SerializeWith<'__a> {
                                    values: (&'__a Option<tokio::time::Duration>,),
                                    phantom: _serde::__private::PhantomData<TicketQuery>,
                                }
                                impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                    fn serialize<__S>(
                                        &self,
                                        __s: __S,
                                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                    where
                                        __S: _serde::Serializer,
                                    {
                                        ::serde_with::As::<
                                            Option<serde_with::DurationSecondsWithFrac<f64>>,
                                        >::serialize(self.values.0, __s)
                                    }
                                }
                                &__SerializeWith {
                                    values: (&self.timeout,),
                                    phantom: _serde::__private::PhantomData::<TicketQuery>,
                                }
                            },
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            /// Implement the [`QueryType`] trait for [`TicketQuery`].
            impl QueryType for TicketQuery {
                fn get_timeout(&self) -> Option<tokio::time::Duration> {
                    self.timeout
                }
            }
            /// A response structure to return the result of a ticket.
            pub struct TicketResponse {
                pub metadata: ResponseMetadata,
                pub ticket: Ticket,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for TicketResponse {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "TicketResponse",
                        "metadata",
                        &self.metadata,
                        "ticket",
                        &&self.ticket,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for TicketResponse {
                #[inline]
                fn clone(&self) -> TicketResponse {
                    TicketResponse {
                        metadata: ::core::clone::Clone::clone(&self.metadata),
                        ticket: ::core::clone::Clone::clone(&self.ticket),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for TicketResponse {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for TicketResponse {
                #[inline]
                fn eq(&self, other: &TicketResponse) -> bool {
                    self.metadata == other.metadata && self.ticket == other.ticket
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for TicketResponse {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<ResponseMetadata>;
                    let _: ::core::cmp::AssertParamIsEq<Ticket>;
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for TicketResponse {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "metadata" => _serde::__private::Ok(__Field::__field0),
                                    "ticket" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"metadata" => _serde::__private::Ok(__Field::__field0),
                                    b"ticket" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<TicketResponse>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = TicketResponse;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct TicketResponse",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    ResponseMetadata,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct TicketResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Ticket,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct TicketResponse with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(TicketResponse {
                                    metadata: __field0,
                                    ticket: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<
                                    ResponseMetadata,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<Ticket> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "metadata",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    ResponseMetadata,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("ticket"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Ticket>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("metadata")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("ticket")?
                                    }
                                };
                                _serde::__private::Ok(TicketResponse {
                                    metadata: __field0,
                                    ticket: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["metadata", "ticket"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "TicketResponse",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<TicketResponse>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for TicketResponse {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "TicketResponse",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "metadata",
                            &self.metadata,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "ticket",
                            &self.ticket,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            impl TicketResponse {
                /// Create a new [`TicketResponse`] instance.
                pub fn new(metadata: ResponseMetadata, ticket: Ticket) -> Self {
                    Self { metadata, ticket }
                }
                /// Create a new [`TicketResponse`] instance with the default metadata.
                pub fn new_from_ticket(
                    start_time: &tokio::time::Instant,
                    ticket: Ticket,
                ) -> Self {
                    Self {
                        metadata: ResponseMetadata::new(start_time),
                        ticket,
                    }
                }
            }
            impl IntoResponse for TicketResponse {
                fn into_response(self) -> axum::response::Response<axum::body::Body> {
                    (
                        axum::http::StatusCode::ACCEPTED,
                        [
                            (axum::http::header::CONTENT_TYPE, "application/json"),
                            (axum::http::header::CACHE_CONTROL, "no-store"),
                        ],
                        axum::Json(self),
                    )
                        .into_response()
                }
            }
        }
        pub use ticket::*;
    }
    pub use message::Ticket;
}
pub mod cli {
    use clap::Parser;
    use crate::CoffeeShopError;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
    /// The default host address for the Waiter, which is to listen on all interfaces.
    const DEFAULT_HOST: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
    /// The default port for the Waiter, which is `7007`.
    const DEFAULT_PORT: u16 = 7007;
    /// The default multicast address for the Announcer.
    const MULTICAST_HOST: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 249);
    /// The default port for the Announcer, which is `65355`.
    const MULTICAST_PORT: u16 = 65355;
    /// The number of Baristas to initiate.
    const DEFAULT_BARISTAS: u16 = 1;
    /// The maximum number of outstanding tickets before the waiter starts rejecting new
    /// requests with a `429 Too Many Requests` status code.
    const MAX_TICKETS: usize = 1024;
    /// Simple program to greet a person
    #[command(version, about, long_about = None)]
    pub struct Config {
        /// The host IP of the server. Defaults to all interfaces.
        #[arg(long, default_value_t = DEFAULT_HOST)]
        pub host: Ipv4Addr,
        /// The port to listen on. Defaults to [`DEFAULT_PORT`].
        #[arg(short, long, default_value_t = DEFAULT_PORT)]
        pub port: u16,
        /// The address to listen for Multicast.
        #[arg(long, default_value_t = MULTICAST_HOST)]
        pub multicast_host: Ipv4Addr,
        /// The port to listen for Multicast.
        #[arg(long, default_value_t = MULTICAST_PORT)]
        pub multicast_port: u16,
        /// The number of Baristas to initiate.
        #[arg(long, default_value_t = DEFAULT_BARISTAS, alias = "workers")]
        pub baristas: u16,
        /// Maximum number of outstanding tickets.
        #[arg(long, default_value_t = MAX_TICKETS)]
        pub max_tickets: usize,
        #[arg(long, default_value = None)]
        pub dynamodb_table: Option<String>,
        #[arg(long, default_value = None)]
        pub sqs_queue: Option<String>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications, clippy::redundant_locals)]
    impl clap::Parser for Config {}
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::CommandFactory for Config {
        fn command<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("coffeeshop");
            <Self as clap::Args>::augment_args(__clap_app)
        }
        fn command_for_update<'b>() -> clap::Command {
            let __clap_app = clap::Command::new("coffeeshop");
            <Self as clap::Args>::augment_args_for_update(__clap_app)
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::FromArgMatches for Config {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = Config {
                host: __clap_arg_matches
                    .remove_one::<Ipv4Addr>("host")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: host",
                    ))?,
                port: __clap_arg_matches
                    .remove_one::<u16>("port")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: port",
                    ))?,
                multicast_host: __clap_arg_matches
                    .remove_one::<Ipv4Addr>("multicast_host")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: multicast_host",
                    ))?,
                multicast_port: __clap_arg_matches
                    .remove_one::<u16>("multicast_port")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: multicast_port",
                    ))?,
                baristas: __clap_arg_matches
                    .remove_one::<u16>("baristas")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: baristas",
                    ))?,
                max_tickets: __clap_arg_matches
                    .remove_one::<usize>("max_tickets")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: max_tickets",
                    ))?,
                dynamodb_table: __clap_arg_matches
                    .remove_one::<String>("dynamodb_table"),
                sqs_queue: __clap_arg_matches.remove_one::<String>("sqs_queue"),
            };
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if __clap_arg_matches.contains_id("host") {
                #[allow(non_snake_case)]
                let host = &mut self.host;
                *host = __clap_arg_matches
                    .remove_one::<Ipv4Addr>("host")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: host",
                    ))?;
            }
            if __clap_arg_matches.contains_id("port") {
                #[allow(non_snake_case)]
                let port = &mut self.port;
                *port = __clap_arg_matches
                    .remove_one::<u16>("port")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: port",
                    ))?;
            }
            if __clap_arg_matches.contains_id("multicast_host") {
                #[allow(non_snake_case)]
                let multicast_host = &mut self.multicast_host;
                *multicast_host = __clap_arg_matches
                    .remove_one::<Ipv4Addr>("multicast_host")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: multicast_host",
                    ))?;
            }
            if __clap_arg_matches.contains_id("multicast_port") {
                #[allow(non_snake_case)]
                let multicast_port = &mut self.multicast_port;
                *multicast_port = __clap_arg_matches
                    .remove_one::<u16>("multicast_port")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: multicast_port",
                    ))?;
            }
            if __clap_arg_matches.contains_id("baristas") {
                #[allow(non_snake_case)]
                let baristas = &mut self.baristas;
                *baristas = __clap_arg_matches
                    .remove_one::<u16>("baristas")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: baristas",
                    ))?;
            }
            if __clap_arg_matches.contains_id("max_tickets") {
                #[allow(non_snake_case)]
                let max_tickets = &mut self.max_tickets;
                *max_tickets = __clap_arg_matches
                    .remove_one::<usize>("max_tickets")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: max_tickets",
                    ))?;
            }
            if __clap_arg_matches.contains_id("dynamodb_table") {
                #[allow(non_snake_case)]
                let dynamodb_table = &mut self.dynamodb_table;
                *dynamodb_table = __clap_arg_matches
                    .remove_one::<String>("dynamodb_table");
            }
            if __clap_arg_matches.contains_id("sqs_queue") {
                #[allow(non_snake_case)]
                let sqs_queue = &mut self.sqs_queue;
                *sqs_queue = __clap_arg_matches.remove_one::<String>("sqs_queue");
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_braces,
        unused_qualifications,
    )]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
        clippy::almost_swapped,
        clippy::redundant_locals,
    )]
    #[automatically_derived]
    impl clap::Args for Config {
        fn group_id() -> Option<clap::Id> {
            Some(clap::Id::from("Config"))
        }
        fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("Config")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 8usize] = [
                                    clap::Id::from("host"),
                                    clap::Id::from("port"),
                                    clap::Id::from("multicast_host"),
                                    clap::Id::from("multicast_port"),
                                    clap::Id::from("baristas"),
                                    clap::Id::from("max_tickets"),
                                    clap::Id::from("dynamodb_table"),
                                    clap::Id::from("sqs_queue"),
                                ];
                                members
                            }),
                    );
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("host")
                            .value_name("HOST")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    Ipv4Addr,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "The host IP of the server. Defaults to all interfaces",
                            )
                            .long_help(None)
                            .long("host")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: Ipv4Addr = DEFAULT_HOST;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("port")
                            .value_name("PORT")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The port to listen on. Defaults to [`DEFAULT_PORT`]")
                            .long_help(None)
                            .short('p')
                            .long("port")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: u16 = DEFAULT_PORT;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("multicast_host")
                            .value_name("MULTICAST_HOST")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    Ipv4Addr,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The address to listen for Multicast")
                            .long_help(None)
                            .long("multicast-host")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: Ipv4Addr = MULTICAST_HOST;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("multicast_port")
                            .value_name("MULTICAST_PORT")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The port to listen for Multicast")
                            .long_help(None)
                            .long("multicast-port")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: u16 = MULTICAST_PORT;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("baristas")
                            .value_name("BARISTAS")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The number of Baristas to initiate")
                            .long_help(None)
                            .long("baristas")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: u16 = DEFAULT_BARISTAS;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            })
                            .alias("workers");
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("max_tickets")
                            .value_name("MAX_TICKETS")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    usize,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("Maximum number of outstanding tickets")
                            .long_help(None)
                            .long("max-tickets")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: usize = MAX_TICKETS;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("dynamodb_table")
                            .value_name("DYNAMODB_TABLE")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.long("dynamodb-table").default_value(None);
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("sqs_queue")
                            .value_name("SQS_QUEUE")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.long("sqs-queue").default_value(None);
                        let arg = arg;
                        arg
                    });
                __clap_app
                    .about("Simple program to greet a person")
                    .long_about(None)
                    .version("0.1.0")
                    .long_about(None)
            }
        }
        fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
            {
                let __clap_app = __clap_app
                    .group(
                        clap::ArgGroup::new("Config")
                            .multiple(true)
                            .args({
                                let members: [clap::Id; 8usize] = [
                                    clap::Id::from("host"),
                                    clap::Id::from("port"),
                                    clap::Id::from("multicast_host"),
                                    clap::Id::from("multicast_port"),
                                    clap::Id::from("baristas"),
                                    clap::Id::from("max_tickets"),
                                    clap::Id::from("dynamodb_table"),
                                    clap::Id::from("sqs_queue"),
                                ];
                                members
                            }),
                    );
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("host")
                            .value_name("HOST")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    Ipv4Addr,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "The host IP of the server. Defaults to all interfaces",
                            )
                            .long_help(None)
                            .long("host")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: Ipv4Addr = DEFAULT_HOST;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("port")
                            .value_name("PORT")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The port to listen on. Defaults to [`DEFAULT_PORT`]")
                            .long_help(None)
                            .short('p')
                            .long("port")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: u16 = DEFAULT_PORT;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("multicast_host")
                            .value_name("MULTICAST_HOST")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    Ipv4Addr,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The address to listen for Multicast")
                            .long_help(None)
                            .long("multicast-host")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: Ipv4Addr = MULTICAST_HOST;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("multicast_port")
                            .value_name("MULTICAST_PORT")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The port to listen for Multicast")
                            .long_help(None)
                            .long("multicast-port")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: u16 = MULTICAST_PORT;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("baristas")
                            .value_name("BARISTAS")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    u16,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The number of Baristas to initiate")
                            .long_help(None)
                            .long("baristas")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: u16 = DEFAULT_BARISTAS;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            })
                            .alias("workers");
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("max_tickets")
                            .value_name("MAX_TICKETS")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    usize,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("Maximum number of outstanding tickets")
                            .long_help(None)
                            .long("max-tickets")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: usize = MAX_TICKETS;
                                        ::std::string::ToString::to_string(&val)
                                    });
                                let s: &'static str = &*s;
                                s
                            });
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("dynamodb_table")
                            .value_name("DYNAMODB_TABLE")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.long("dynamodb-table").default_value(None);
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("sqs_queue")
                            .value_name("SQS_QUEUE")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg.long("sqs-queue").default_value(None);
                        let arg = arg.required(false);
                        arg
                    });
                __clap_app
                    .about("Simple program to greet a person")
                    .long_about(None)
                    .version("0.1.0")
                    .long_about(None)
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Config {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "host",
                "port",
                "multicast_host",
                "multicast_port",
                "baristas",
                "max_tickets",
                "dynamodb_table",
                "sqs_queue",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.host,
                &self.port,
                &self.multicast_host,
                &self.multicast_port,
                &self.baristas,
                &self.max_tickets,
                &self.dynamodb_table,
                &&self.sqs_queue,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Config",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Config {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Config {
        #[inline]
        fn eq(&self, other: &Config) -> bool {
            self.host == other.host && self.port == other.port
                && self.multicast_host == other.multicast_host
                && self.multicast_port == other.multicast_port
                && self.baristas == other.baristas
                && self.max_tickets == other.max_tickets
                && self.dynamodb_table == other.dynamodb_table
                && self.sqs_queue == other.sqs_queue
        }
    }
    impl Default for Config {
        /// Get the args with the default configuration.
        ///
        /// This allows [`Config`] to be used without parsing the CLI args. This is useful
        /// when this framework is not implemented as a CLI tool, and the configurations are
        /// sourced from elsewhere.
        fn default() -> Self {
            Self {
                host: DEFAULT_HOST,
                port: DEFAULT_PORT,
                multicast_host: MULTICAST_HOST,
                multicast_port: MULTICAST_PORT,
                baristas: DEFAULT_BARISTAS,
                max_tickets: MAX_TICKETS,
                dynamodb_table: None,
                sqs_queue: None,
            }
        }
    }
    impl Config {
        /// Instantiate a new [`Config`] with [`Self::default`] settings.
        pub fn new() -> Self {
            Self::default()
        }
        /// Check if the multicast address is correct; if not, consume itself and
        /// return an [`Err`].
        fn validate_multicast_addr(self) -> Result<Self, CoffeeShopError> {
            let ip_addr = IpAddr::V4(self.multicast_host);
            if ip_addr.is_multicast() {
                Ok(self)
            } else {
                Err(CoffeeShopError::InvalidMulticastAddress(ip_addr))
            }
        }
        /// Builder pattern - change the Waiter address.
        pub fn with_host_addr(mut self, addr: SocketAddrV4) -> Self {
            self.port = addr.port();
            self.host = *addr.ip();
            self
        }
        /// Builder pattern - change the multicast address.
        pub fn with_multicast_addr(
            mut self,
            addr: SocketAddrV4,
        ) -> Result<Self, CoffeeShopError> {
            self.multicast_port = addr.port();
            self.multicast_host = *addr.ip();
            self.validate_multicast_addr()
        }
        /// Builder pattern - change the number of baristas to initiate.
        pub fn with_baristas(mut self, count: u16) -> Result<Self, CoffeeShopError> {
            if count == 0 {
                Err(CoffeeShopError::InvalidConfiguration {
                    field: "baristas",
                    value: ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("must be positive number, found {0}.", count),
                        );
                        res
                    }),
                })
            } else {
                self.baristas = count.max(1);
                Ok(self)
            }
        }
        /// Builder pattern - change the maximum number of tickets.
        pub fn with_max_tickets(
            mut self,
            count: usize,
        ) -> Result<Self, CoffeeShopError> {
            if count == 0 {
                Err(CoffeeShopError::InvalidConfiguration {
                    field: "max_tickets",
                    value: ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("must be positive number, found {0}.", count),
                        );
                        res
                    }),
                })
            } else {
                self.max_tickets = count;
                Ok(self)
            }
        }
    }
    impl Config {
        /// Get the Multicast address in a packaged [`SocketAddr`] instance.
        pub fn multicast_addr(&self) -> SocketAddr {
            SocketAddr::new(IpAddr::V4(self.multicast_host), self.multicast_port)
        }
        /// Get the host address in a packaged [`SocketAddr`] instance.
        pub fn host_addr(&self) -> SocketAddr {
            SocketAddr::new(IpAddr::V4(self.host), self.port)
        }
    }
}
mod logger {
    //! Centralised logging for the coffee shop.
    //!
    #[cfg(feature = "debug")]
    use std::sync::OnceLock;
    /// Lock for initialising the logger.
    #[cfg(feature = "debug")]
    static INIT: OnceLock<()> = OnceLock::new();
    /// Initialises the logger.
    pub fn init() {
        #[cfg(feature = "debug")] INIT.get_or_init(env_logger::init);
    }
}
