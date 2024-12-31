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
pub mod errors {
    use axum::{body::Body, http, response::IntoResponse, Json};
    use thiserror::Error;
    use std::net::{IpAddr, SocketAddr};
    use crate::models::Ticket;
    /// Re-exports necessary for the error handling of SQS SDK.
    mod sqs {
        pub const DEFAULT_ERROR_MESSAGE: &str = "(No details provided)";
        pub use aws_sdk_sqs::operation::send_message::SendMessageError;
        pub use aws_sdk_sqs::types::error::*;
    }
    /// The error type for validation errors.
    ///
    /// This is a simple key-value pair where the key is the field that failed validation,
    /// and the value is the error message.
    ///
    /// The original value is not included in the error, as it could violate lifetimes
    /// as well as privacy.
    ///
    /// By convention, if the error relates to the whole of
    /// - query parameters, the key should be ``$query``, and
    /// - the request body, the key should be ``$body``.
    pub type ValidationError = std::collections::HashMap<String, String>;
    /// The error type for exporting any error that occurs in this crate.
    ///
    /// Since the [`Barista`]s have to serialize any errors to DynamoDB before a
    /// [`Waiter`] can retrieve it, we need a standardised error type to ensure
    /// that the errors can be logically
    #[non_exhaustive]
    pub struct ErrorSchema {
        /// The HTTP status code to send to the client in the response.
        #[serde(with = "http_serde::status_code")]
        status_code: http::StatusCode,
        /// An identifier for the type of error that occurred in PascalCase, e.g.
        /// `InvalidConfiguration`.
        error: String,
        /// Additional details for the error.
        ///
        /// These are returned to the user directly as part of the error response.
        /// This crate will not attempt to interpret the contents of this field.
        ///
        /// It is encouraged for this field to contain the key "message" with a human-readable
        /// error message.
        details: Option<serde_json::Value>,
    }
    #[allow(unused_qualifications)]
    #[automatically_derived]
    impl ::thiserror::__private::Error for ErrorSchema {}
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorSchema {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ErrorSchema",
                "status_code",
                &self.status_code,
                "error",
                &self.error,
                "details",
                &&self.details,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorSchema {
        #[inline]
        fn clone(&self) -> ErrorSchema {
            ErrorSchema {
                status_code: ::core::clone::Clone::clone(&self.status_code),
                error: ::core::clone::Clone::clone(&self.error),
                details: ::core::clone::Clone::clone(&self.details),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ErrorSchema {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ErrorSchema {
        #[inline]
        fn eq(&self, other: &ErrorSchema) -> bool {
            self.status_code == other.status_code && self.error == other.error
                && self.details == other.details
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ErrorSchema {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ErrorSchema",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "status_code",
                    {
                        #[doc(hidden)]
                        struct __SerializeWith<'__a> {
                            values: (&'__a http::StatusCode,),
                            phantom: _serde::__private::PhantomData<ErrorSchema>,
                        }
                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                            fn serialize<__S>(
                                &self,
                                __s: __S,
                            ) -> _serde::__private::Result<__S::Ok, __S::Error>
                            where
                                __S: _serde::Serializer,
                            {
                                http_serde::status_code::serialize(self.values.0, __s)
                            }
                        }
                        &__SerializeWith {
                            values: (&self.status_code,),
                            phantom: _serde::__private::PhantomData::<ErrorSchema>,
                        }
                    },
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "error",
                    &self.error,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "details",
                    &self.details,
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
        impl<'de> _serde::Deserialize<'de> for ErrorSchema {
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
                            "status_code" => _serde::__private::Ok(__Field::__field0),
                            "error" => _serde::__private::Ok(__Field::__field1),
                            "details" => _serde::__private::Ok(__Field::__field2),
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
                            b"status_code" => _serde::__private::Ok(__Field::__field0),
                            b"error" => _serde::__private::Ok(__Field::__field1),
                            b"details" => _serde::__private::Ok(__Field::__field2),
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
                    marker: _serde::__private::PhantomData<ErrorSchema>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ErrorSchema;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ErrorSchema",
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
                        let __field0 = match {
                            #[doc(hidden)]
                            struct __DeserializeWith<'de> {
                                value: http::StatusCode,
                                phantom: _serde::__private::PhantomData<ErrorSchema>,
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
                                        value: http_serde::status_code::deserialize(
                                            __deserializer,
                                        )?,
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
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ErrorSchema with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ErrorSchema with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<serde_json::Value>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct ErrorSchema with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ErrorSchema {
                            status_code: __field0,
                            error: __field1,
                            details: __field2,
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
                        let mut __field0: _serde::__private::Option<http::StatusCode> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            Option<serde_json::Value>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "status_code",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some({
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: http::StatusCode,
                                            phantom: _serde::__private::PhantomData<ErrorSchema>,
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
                                                    value: http_serde::status_code::deserialize(
                                                        __deserializer,
                                                    )?,
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
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("error"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "details",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<serde_json::Value>,
                                        >(&mut __map)?,
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
                                return _serde::__private::Err(
                                    <__A::Error as _serde::de::Error>::missing_field(
                                        "status_code",
                                    ),
                                );
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("error")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("details")?
                            }
                        };
                        _serde::__private::Ok(ErrorSchema {
                            status_code: __field0,
                            error: __field1,
                            details: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "status_code",
                    "error",
                    "details",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ErrorSchema",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ErrorSchema>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    /// The error type for the Coffee Machine.
    ///
    /// This is for downstream implementers to use as the error type for their Coffee Machine.
    pub type CoffeeMachineError = ErrorSchema;
    impl ErrorSchema {
        /// Create a new instance of [`ErrorSchema`].
        pub fn new(
            status_code: http::StatusCode,
            error: String,
            details: Option<serde_json::Value>,
        ) -> Self {
            Self {
                status_code,
                error,
                details,
            }
        }
    }
    impl IntoResponse for ErrorSchema {
        fn into_response(self) -> axum::response::Response<Body> {
            (
                self.status_code,
                [
                    (http::header::CONTENT_TYPE, "application/json"),
                    (http::header::CACHE_CONTROL, "no-store"),
                ],
                Json(
                    serde_json::to_value(&self)
                        .expect(
                            "Failed to serialize the `ErrorSchema` into JSON for the response. This should not be possible; please check your error type definition.",
                        ),
                ),
            )
                .into_response()
        }
    }
    impl std::fmt::Display for ErrorSchema {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                format_args!(
                    "{0}: {1}",
                    self
                        .status_code
                        .canonical_reason()
                        .unwrap_or(
                            &::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0:?}", self.status_code.as_u16()),
                                );
                                res
                            }),
                        ),
                    self.error,
                ),
            )
        }
    }
    pub enum CoffeeShopError {
        #[error("Invalid configuration for {field}: {message}")]
        InvalidConfiguration { field: &'static str, message: String },
        #[error("{0:?} is not a valid multicast address.")]
        InvalidMulticastAddress(IpAddr),
        #[error("Received an invalid MulticastMessage from {addr}.")]
        InvalidMulticastMessage {
            data: Vec<u8>,
            addr: String,
            error: prost::DecodeError,
        },
        #[error("HTTP Host failed: {0}")]
        HTTPServerError(std::io::ErrorKind, std::io::Error),
        #[error("Failed to bind listener to socket address {1}: {0}")]
        ListenerCreationFailure(String, SocketAddr),
        #[error("Could not serialize the payload: {0}")]
        ResultBinaryConversionError(#[from] Box<bincode::ErrorKind>),
        #[error("Could not compress/decompress the payload: {0}")]
        ResultBinaryCompressionError(#[from] gzp::GzpError),
        #[error("Temporary directory could not be created: {0}")]
        TempDirCreationFailure(String),
        #[error("Temporary file access failure at {path}: {reason}")]
        TempFileAccessFailure { path: std::path::PathBuf, reason: String },
        #[error(
            "The path for a temporary file is non-uniquely generated; this is improbable unless cleanup is not working. Please verify."
        )]
        NonUniqueTemporaryFile,
        #[error("Failed to decode from Base64: {0}")]
        Base64DecodingError(#[from] base64::DecodeError),
        #[error(
            "The requested payload is {0} bytes in size, exceeding the limit; try chunking the payload and retry the request."
        )]
        Base64EncodingOversize(usize),
        #[error("An IOError::{0} had occurred: {1}")]
        IOError(std::io::ErrorKind, std::io::Error),
        #[error("An IOError::{0} had occurred during multicast operations: {1}")]
        MulticastIOError(std::io::ErrorKind, std::io::Error),
        #[error("Timed out awaiting results after {0:?} seconds")]
        RetrieveTimeout(tokio::time::Duration),
        #[error("An error relating to AWS IAM credentials occurred: {0}")]
        AWSCredentialsError(String),
        #[error("AWS Configuration incomplete: {0}")]
        AWSConfigIncomplete(String),
        #[error(
            "The specified AWS SQS queue URL does not exists. Please verify the URL: {0}"
        )]
        AWSQueueDoesNotExist(String),
        #[error(
            "AWS SQS Rejected the message: {0}; please verify the payload and try again."
        )]
        AWSSQSInvalidMessage(String),
        #[error("AWS SQS Queue is empty after waiting for {0:?}.")]
        AWSSQSQueueEmpty(tokio::time::Duration),
        #[error("Unexpected AWS SQS Send Message Error: {0:?}")]
        AWSSQSSendMessageError(#[from] Box<sqs::SendMessageError>),
        #[error(
            "Message from AWS SQS had already been completed, and cannot be {0} again."
        )]
        AWSSQSStagedReceiptAlreadyCompleted(&'static str),
        #[error("AWS responded with unexpected data: {0}")]
        UnexpectedAWSResponse(String),
        /// Generic AWS SDK error.
        ///
        /// Use this as a last resort, as it is not specific to any SDK.
        #[error("AWS SDK Error: {0}")]
        AWSSdkError(String),
        #[error("Error during processing: {0}")]
        ProcessingError(#[from] CoffeeMachineError),
        #[error("Result is already set, cannot set again.")]
        ResultAlreadySet,
        #[error(
            "The ticket {0} does not have a result. It could have been purged, or the ticket is invalid."
        )]
        ResultNotFound(Ticket),
        #[error("The ticket {0} was not found.")]
        TicketNotFound(Ticket),
        #[error("Upstream worker reported an error: {0:?}")]
        ErrorSchema(ErrorSchema),
        #[error("DynamoDB item is found malformed: {0}")]
        DynamoDBMalformedItem(String),
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
                CoffeeShopError::HTTPServerError { .. } => ::core::option::Option::None,
                CoffeeShopError::ListenerCreationFailure { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::ResultBinaryConversionError { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                CoffeeShopError::ResultBinaryCompressionError { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                CoffeeShopError::TempDirCreationFailure { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::TempFileAccessFailure { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::NonUniqueTemporaryFile { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::Base64DecodingError { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                CoffeeShopError::Base64EncodingOversize { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::IOError { .. } => ::core::option::Option::None,
                CoffeeShopError::MulticastIOError { .. } => ::core::option::Option::None,
                CoffeeShopError::RetrieveTimeout { .. } => ::core::option::Option::None,
                CoffeeShopError::AWSCredentialsError { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::AWSConfigIncomplete { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::AWSQueueDoesNotExist { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::AWSSQSInvalidMessage { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::AWSSQSQueueEmpty { .. } => ::core::option::Option::None,
                CoffeeShopError::AWSSQSSendMessageError { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                CoffeeShopError::AWSSQSStagedReceiptAlreadyCompleted { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::UnexpectedAWSResponse { .. } => {
                    ::core::option::Option::None
                }
                CoffeeShopError::AWSSdkError { .. } => ::core::option::Option::None,
                CoffeeShopError::ProcessingError { 0: source, .. } => {
                    ::core::option::Option::Some(source.as_dyn_error())
                }
                CoffeeShopError::ResultAlreadySet { .. } => ::core::option::Option::None,
                CoffeeShopError::ResultNotFound { .. } => ::core::option::Option::None,
                CoffeeShopError::TicketNotFound { .. } => ::core::option::Option::None,
                CoffeeShopError::ErrorSchema { .. } => ::core::option::Option::None,
                CoffeeShopError::DynamoDBMalformedItem { .. } => {
                    ::core::option::Option::None
                }
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
                CoffeeShopError::InvalidConfiguration { field, message } => {
                    match (field.as_display(), message.as_display()) {
                        (__display_field, __display_message) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Invalid configuration for {0}: {1}",
                                        __display_field,
                                        __display_message,
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
                CoffeeShopError::InvalidMulticastMessage { data, addr, error } => {
                    match (addr.as_display(),) {
                        (__display_addr,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Received an invalid MulticastMessage from {0}.",
                                        __display_addr,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::HTTPServerError(_0, _1) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!("HTTP Host failed: {0}", __display0),
                                )
                        }
                    }
                }
                CoffeeShopError::ListenerCreationFailure(_0, _1) => {
                    match (_1.as_display(), _0.as_display()) {
                        (__display1, __display0) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Failed to bind listener to socket address {0}: {1}",
                                        __display1,
                                        __display0,
                                    ),
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
                CoffeeShopError::TempDirCreationFailure(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Temporary directory could not be created: {0}",
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
                CoffeeShopError::Base64DecodingError(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Failed to decode from Base64: {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::Base64EncodingOversize(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "The requested payload is {0} bytes in size, exceeding the limit; try chunking the payload and retry the request.",
                                        __display0,
                                    ),
                                )
                        }
                    }
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
                CoffeeShopError::MulticastIOError(_0, _1) => {
                    match (_0.as_display(), _1.as_display()) {
                        (__display0, __display1) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "An IOError::{0} had occurred during multicast operations: {1}",
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
                CoffeeShopError::AWSCredentialsError(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "An error relating to AWS IAM credentials occurred: {0}",
                                        __display0,
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
                CoffeeShopError::AWSQueueDoesNotExist(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "The specified AWS SQS queue URL does not exists. Please verify the URL: {0}",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::AWSSQSInvalidMessage(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "AWS SQS Rejected the message: {0}; please verify the payload and try again.",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::AWSSQSQueueEmpty(_0) => {
                    match (_0,) {
                        (__field0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "AWS SQS Queue is empty after waiting for {0:?}.",
                                        __field0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::AWSSQSSendMessageError(_0) => {
                    match (_0,) {
                        (__field0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Unexpected AWS SQS Send Message Error: {0:?}",
                                        __field0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::AWSSQSStagedReceiptAlreadyCompleted(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Message from AWS SQS had already been completed, and cannot be {0} again.",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::UnexpectedAWSResponse(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "AWS responded with unexpected data: {0}",
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
                CoffeeShopError::ProcessingError(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!("Error during processing: {0}", __display0),
                                )
                        }
                    }
                }
                CoffeeShopError::ResultAlreadySet {} => {
                    __formatter.write_str("Result is already set, cannot set again.")
                }
                CoffeeShopError::ResultNotFound(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "The ticket {0} does not have a result. It could have been purged, or the ticket is invalid.",
                                        __display0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::TicketNotFound(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!("The ticket {0} was not found.", __display0),
                                )
                        }
                    }
                }
                CoffeeShopError::ErrorSchema(_0) => {
                    match (_0,) {
                        (__field0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "Upstream worker reported an error: {0:?}",
                                        __field0,
                                    ),
                                )
                        }
                    }
                }
                CoffeeShopError::DynamoDBMalformedItem(_0) => {
                    match (_0.as_display(),) {
                        (__display0,) => {
                            __formatter
                                .write_fmt(
                                    format_args!(
                                        "DynamoDB item is found malformed: {0}",
                                        __display0,
                                    ),
                                )
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
    #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
    #[automatically_derived]
    impl ::core::convert::From<base64::DecodeError> for CoffeeShopError {
        fn from(source: base64::DecodeError) -> Self {
            CoffeeShopError::Base64DecodingError {
                0: source,
            }
        }
    }
    #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
    #[automatically_derived]
    impl ::core::convert::From<Box<sqs::SendMessageError>> for CoffeeShopError {
        fn from(source: Box<sqs::SendMessageError>) -> Self {
            CoffeeShopError::AWSSQSSendMessageError {
                0: source,
            }
        }
    }
    #[allow(deprecated, unused_qualifications, clippy::needless_lifetimes)]
    #[automatically_derived]
    impl ::core::convert::From<CoffeeMachineError> for CoffeeShopError {
        fn from(source: CoffeeMachineError) -> Self {
            CoffeeShopError::ProcessingError {
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
                    message: __self_1,
                } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "InvalidConfiguration",
                        "field",
                        __self_0,
                        "message",
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
                    data: __self_0,
                    addr: __self_1,
                    error: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "InvalidMulticastMessage",
                        "data",
                        __self_0,
                        "addr",
                        __self_1,
                        "error",
                        &__self_2,
                    )
                }
                CoffeeShopError::HTTPServerError(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "HTTPServerError",
                        __self_0,
                        &__self_1,
                    )
                }
                CoffeeShopError::ListenerCreationFailure(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "ListenerCreationFailure",
                        __self_0,
                        &__self_1,
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
                CoffeeShopError::TempDirCreationFailure(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TempDirCreationFailure",
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
                CoffeeShopError::Base64DecodingError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Base64DecodingError",
                        &__self_0,
                    )
                }
                CoffeeShopError::Base64EncodingOversize(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Base64EncodingOversize",
                        &__self_0,
                    )
                }
                CoffeeShopError::IOError(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "IOError",
                        __self_0,
                        &__self_1,
                    )
                }
                CoffeeShopError::MulticastIOError(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "MulticastIOError",
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
                CoffeeShopError::AWSCredentialsError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AWSCredentialsError",
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
                CoffeeShopError::AWSQueueDoesNotExist(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AWSQueueDoesNotExist",
                        &__self_0,
                    )
                }
                CoffeeShopError::AWSSQSInvalidMessage(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AWSSQSInvalidMessage",
                        &__self_0,
                    )
                }
                CoffeeShopError::AWSSQSQueueEmpty(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AWSSQSQueueEmpty",
                        &__self_0,
                    )
                }
                CoffeeShopError::AWSSQSSendMessageError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AWSSQSSendMessageError",
                        &__self_0,
                    )
                }
                CoffeeShopError::AWSSQSStagedReceiptAlreadyCompleted(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AWSSQSStagedReceiptAlreadyCompleted",
                        &__self_0,
                    )
                }
                CoffeeShopError::UnexpectedAWSResponse(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "UnexpectedAWSResponse",
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
                CoffeeShopError::ProcessingError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ProcessingError",
                        &__self_0,
                    )
                }
                CoffeeShopError::ResultAlreadySet => {
                    ::core::fmt::Formatter::write_str(f, "ResultAlreadySet")
                }
                CoffeeShopError::ResultNotFound(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ResultNotFound",
                        &__self_0,
                    )
                }
                CoffeeShopError::TicketNotFound(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TicketNotFound",
                        &__self_0,
                    )
                }
                CoffeeShopError::ErrorSchema(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ErrorSchema",
                        &__self_0,
                    )
                }
                CoffeeShopError::DynamoDBMalformedItem(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "DynamoDBMalformedItem",
                        &__self_0,
                    )
                }
            }
        }
    }
    impl ::core::convert::From<CoffeeShopError> for &'static str {
        fn from(x: CoffeeShopError) -> &'static str {
            match x {
                CoffeeShopError::InvalidConfiguration { .. } => "InvalidConfiguration",
                CoffeeShopError::InvalidMulticastAddress(..) => "InvalidMulticastAddress",
                CoffeeShopError::InvalidMulticastMessage { .. } => {
                    "InvalidMulticastMessage"
                }
                CoffeeShopError::HTTPServerError(..) => "HTTPServerError",
                CoffeeShopError::ListenerCreationFailure(..) => "ListenerCreationFailure",
                CoffeeShopError::ResultBinaryConversionError(..) => {
                    "ResultBinaryConversionError"
                }
                CoffeeShopError::ResultBinaryCompressionError(..) => {
                    "ResultBinaryCompressionError"
                }
                CoffeeShopError::TempDirCreationFailure(..) => "TempDirCreationFailure",
                CoffeeShopError::TempFileAccessFailure { .. } => "TempFileAccessFailure",
                CoffeeShopError::NonUniqueTemporaryFile => "NonUniqueTemporaryFile",
                CoffeeShopError::Base64DecodingError(..) => "Base64DecodingError",
                CoffeeShopError::Base64EncodingOversize(..) => "Base64EncodingOversize",
                CoffeeShopError::IOError(..) => "IOError",
                CoffeeShopError::MulticastIOError(..) => "MulticastIOError",
                CoffeeShopError::RetrieveTimeout(..) => "RetrieveTimeout",
                CoffeeShopError::AWSCredentialsError(..) => "AWSCredentialsError",
                CoffeeShopError::AWSConfigIncomplete(..) => "AWSConfigIncomplete",
                CoffeeShopError::AWSQueueDoesNotExist(..) => "AWSQueueDoesNotExist",
                CoffeeShopError::AWSSQSInvalidMessage(..) => "AWSSQSInvalidMessage",
                CoffeeShopError::AWSSQSQueueEmpty(..) => "AWSSQSQueueEmpty",
                CoffeeShopError::AWSSQSSendMessageError(..) => "AWSSQSSendMessageError",
                CoffeeShopError::AWSSQSStagedReceiptAlreadyCompleted(..) => {
                    "AWSSQSStagedReceiptAlreadyCompleted"
                }
                CoffeeShopError::UnexpectedAWSResponse(..) => "UnexpectedAWSResponse",
                CoffeeShopError::AWSSdkError(..) => "AWSSdkError",
                CoffeeShopError::ProcessingError(..) => "ProcessingError",
                CoffeeShopError::ResultAlreadySet => "ResultAlreadySet",
                CoffeeShopError::ResultNotFound(..) => "ResultNotFound",
                CoffeeShopError::TicketNotFound(..) => "TicketNotFound",
                CoffeeShopError::ErrorSchema(..) => "ErrorSchema",
                CoffeeShopError::DynamoDBMalformedItem(..) => "DynamoDBMalformedItem",
            }
        }
    }
    impl<'_derivative_strum> ::core::convert::From<&'_derivative_strum CoffeeShopError>
    for &'static str {
        fn from(x: &'_derivative_strum CoffeeShopError) -> &'static str {
            match *x {
                CoffeeShopError::InvalidConfiguration { .. } => "InvalidConfiguration",
                CoffeeShopError::InvalidMulticastAddress(..) => "InvalidMulticastAddress",
                CoffeeShopError::InvalidMulticastMessage { .. } => {
                    "InvalidMulticastMessage"
                }
                CoffeeShopError::HTTPServerError(..) => "HTTPServerError",
                CoffeeShopError::ListenerCreationFailure(..) => "ListenerCreationFailure",
                CoffeeShopError::ResultBinaryConversionError(..) => {
                    "ResultBinaryConversionError"
                }
                CoffeeShopError::ResultBinaryCompressionError(..) => {
                    "ResultBinaryCompressionError"
                }
                CoffeeShopError::TempDirCreationFailure(..) => "TempDirCreationFailure",
                CoffeeShopError::TempFileAccessFailure { .. } => "TempFileAccessFailure",
                CoffeeShopError::NonUniqueTemporaryFile => "NonUniqueTemporaryFile",
                CoffeeShopError::Base64DecodingError(..) => "Base64DecodingError",
                CoffeeShopError::Base64EncodingOversize(..) => "Base64EncodingOversize",
                CoffeeShopError::IOError(..) => "IOError",
                CoffeeShopError::MulticastIOError(..) => "MulticastIOError",
                CoffeeShopError::RetrieveTimeout(..) => "RetrieveTimeout",
                CoffeeShopError::AWSCredentialsError(..) => "AWSCredentialsError",
                CoffeeShopError::AWSConfigIncomplete(..) => "AWSConfigIncomplete",
                CoffeeShopError::AWSQueueDoesNotExist(..) => "AWSQueueDoesNotExist",
                CoffeeShopError::AWSSQSInvalidMessage(..) => "AWSSQSInvalidMessage",
                CoffeeShopError::AWSSQSQueueEmpty(..) => "AWSSQSQueueEmpty",
                CoffeeShopError::AWSSQSSendMessageError(..) => "AWSSQSSendMessageError",
                CoffeeShopError::AWSSQSStagedReceiptAlreadyCompleted(..) => {
                    "AWSSQSStagedReceiptAlreadyCompleted"
                }
                CoffeeShopError::UnexpectedAWSResponse(..) => "UnexpectedAWSResponse",
                CoffeeShopError::AWSSdkError(..) => "AWSSdkError",
                CoffeeShopError::ProcessingError(..) => "ProcessingError",
                CoffeeShopError::ResultAlreadySet => "ResultAlreadySet",
                CoffeeShopError::ResultNotFound(..) => "ResultNotFound",
                CoffeeShopError::TicketNotFound(..) => "TicketNotFound",
                CoffeeShopError::ErrorSchema(..) => "ErrorSchema",
                CoffeeShopError::DynamoDBMalformedItem(..) => "DynamoDBMalformedItem",
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
        /// Convenient method to create a [`CoffeeShopError::MulticastIOError`] variant from [`std::io::Error`].
        pub fn from_multicast_io_error(error: std::io::Error) -> Self {
            CoffeeShopError::MulticastIOError(error.kind(), error)
        }
        /// Convenient method to create a [`CoffeeShopError::HTTPServerError`] variant from [`std::io::Error`].
        pub fn from_server_io_error(error: std::io::Error) -> Self {
            CoffeeShopError::HTTPServerError(error.kind(), error)
        }
        /// Convenient method to map AWS SQS SDK errors to [`CoffeeShopError`].
        pub fn from_aws_sqs_send_message_error(error: sqs::SendMessageError) -> Self {
            match error {
                sqs::SendMessageError::QueueDoesNotExist(
                    sqs::QueueDoesNotExist { message: msg_opt, .. },
                ) => {
                    CoffeeShopError::AWSQueueDoesNotExist(
                        msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
                    )
                }
                sqs::SendMessageError::InvalidMessageContents(
                    sqs::InvalidMessageContents { message: msg_opt, .. },
                ) => {
                    CoffeeShopError::AWSSQSInvalidMessage(
                        msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
                    )
                }
                sqs::SendMessageError::InvalidAddress(
                    sqs::InvalidAddress { message: msg_opt, .. },
                ) => {
                    CoffeeShopError::InvalidConfiguration {
                        field: "sqs_queue",
                        message: msg_opt
                            .unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
                    }
                }
                sqs::SendMessageError::KmsAccessDenied(
                    sqs::KmsAccessDenied { message: msg_opt, .. },
                ) => {
                    CoffeeShopError::AWSCredentialsError(
                        msg_opt.unwrap_or_else(|| sqs::DEFAULT_ERROR_MESSAGE.to_string()),
                    )
                }
                err => CoffeeShopError::AWSSQSSendMessageError(Box::new(err)),
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
                CoffeeShopError::AWSConfigIncomplete(_) => http::StatusCode::UNAUTHORIZED,
                CoffeeShopError::AWSQueueDoesNotExist(_) => http::StatusCode::BAD_GATEWAY,
                CoffeeShopError::InvalidConfiguration { .. } => {
                    http::StatusCode::INTERNAL_SERVER_ERROR
                }
                CoffeeShopError::InvalidMulticastAddress(_) => {
                    http::StatusCode::BAD_REQUEST
                }
                CoffeeShopError::InvalidMulticastMessage { .. } => {
                    http::StatusCode::BAD_REQUEST
                }
                CoffeeShopError::RetrieveTimeout(_) => http::StatusCode::REQUEST_TIMEOUT,
                CoffeeShopError::Base64EncodingOversize(_) => {
                    http::StatusCode::PAYLOAD_TOO_LARGE
                }
                CoffeeShopError::ProcessingError(ErrorSchema { status_code, .. }) => {
                    *status_code
                }
                CoffeeShopError::ErrorSchema(ErrorSchema { status_code, .. }) => {
                    *status_code
                }
                CoffeeShopError::DynamoDBMalformedItem(_) => {
                    http::StatusCode::BAD_GATEWAY
                }
                _ => http::StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
        /// This method returns the kind of error as a string.
        pub fn kind(&self) -> &'static str {
            self.into()
        }
        pub fn as_error_schema(&self) -> ErrorSchema {
            match self {
                Self::ProcessingError(err) => err.clone(),
                Self::ErrorSchema(err) => err.clone(),
                _ => {
                    ErrorSchema::new(
                        self.status_code(),
                        self.kind().to_string(),
                        Some(
                            ::serde_json::Value::Object({
                                let mut object = ::serde_json::Map::new();
                                let _ = object
                                    .insert(
                                        ("message").into(),
                                        ::serde_json::to_value(&self.to_string()).unwrap(),
                                    );
                                object
                            }),
                        ),
                    )
                }
            }
        }
        /// Converts the error into a JSON object.
        pub fn as_json(&self) -> serde_json::Value {
            serde_json::to_value(self.as_error_schema())
                .unwrap_or_else(|_| {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "Failed to serialize the `ErrorSchema` into JSON for the response. This should not be possible; please check your error type definition: {0:?}",
                            self,
                        ),
                    );
                })
        }
    }
    impl PartialEq for CoffeeShopError {
        fn eq(&self, other: &Self) -> bool {
            (self.kind() == other.kind() || self.kind() == "ErrorSchema"
                || other.kind() == "ErrorSchema") && self.as_json() == other.as_json()
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
                Json(self.as_json()),
            )
                .into_response()
        }
    }
}
pub use errors::{CoffeeMachineError, CoffeeShopError, ValidationError};
/// Re-export the necessary crates for implementors of [`models::Machine`].
///
/// This module is intended to be used by implementors of the [`models::Machine`] trait
/// to ensure that the versions of the compatible dependencies are accessible for the
/// downstream implementors.
pub mod reexports {
    /// Re-export the `async_trait` crate so that implementors of [`models::Machine`]
    /// can use it without concerns for mismatched versions.
    pub use async_trait;
    /// Re-export the `axum` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use axum;
    /// Re-export the `serde` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use serde;
    /// Re-export the `serde_json` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use serde_json;
    /// Re-export the `uuid` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use uuid;
    /// Re-export the `tokio` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use socket2;
    /// Re-export the `tokio` crate so that implementors of [`models::Machine`] can use it
    /// without concerns for mismatched versions.
    pub use tokio_socket2;
}
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
        /// A trait indicating that the implementing struct has an AWS SDK configuration.
        pub trait HasAWSSdkConfig: Send + Sync {
            /// Get the AWS configuration.
            fn aws_config(&self) -> &SdkConfig;
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
        /// The key for the status of the processing result.
        const SUCCESS_KEY: &str = "success";
        /// The key for the status code of the processing result.
        const STATUS_KEY: &str = "status_code";
        /// The key for the output of the processing result.
        const OUTPUT_KEY: &str = "output";
        /// The key for the error of the processing result.
        const ERROR_KEY: &str = "error";
        /// The key for the time-to-live of the processing result.
        const TTL_KEY: &str = "ttl";
        mod config {
            use crate::helpers::aws::HasAWSSdkConfig;
            use crate::helpers::aws;
            /// A [`HasDynamoDBConfiguration`] contains the configuration for the DynamoDB table
            /// that the [`Shop`] will be using.
            pub trait HasDynamoDBConfiguration: HasAWSSdkConfig {
                /// The name of the DynamoDB table.
                fn dynamodb_table(&self) -> &str;
                /// The partition key of the DynamoDB table.
                fn dynamodb_partition_key(&self) -> &str;
                /// The time-to-live (TTL) duration for the items in the DynamoDB table.
                fn dynamodb_ttl(&self) -> tokio::time::Duration;
                /// Extract the configuration as a separate struct.
                ///
                /// This is useful if the main configuration struct is too large, or it
                /// lacks certain traits such as [`Send`] or [`Sync`].
                fn dynamodb_configuration(&self) -> DynamoDBConfiguration {
                    DynamoDBConfiguration {
                        table: self.dynamodb_table().to_owned(),
                        partition_key: self.dynamodb_partition_key().to_owned(),
                        ttl: self.dynamodb_ttl(),
                        aws_config: self.aws_config().clone(),
                    }
                }
            }
            /// A minimal implementation of [`HasDynamoDBConfiguration`] for testing purposes, or
            /// to use this module without a full [`Shop`] configuration.
            pub struct DynamoDBConfiguration {
                pub table: String,
                pub partition_key: String,
                pub ttl: tokio::time::Duration,
                pub aws_config: aws::SdkConfig,
            }
            impl HasAWSSdkConfig for DynamoDBConfiguration {
                fn aws_config(&self) -> &aws::SdkConfig {
                    &self.aws_config
                }
            }
            impl HasDynamoDBConfiguration for DynamoDBConfiguration {
                fn dynamodb_table(&self) -> &str {
                    &self.table
                }
                fn dynamodb_partition_key(&self) -> &str {
                    &self.partition_key
                }
                fn dynamodb_ttl(&self) -> tokio::time::Duration {
                    self.ttl
                }
            }
        }
        pub use config::*;
        mod process_result_to_item {
            //! This module contains implementations on
            //! [`dynamodb::operation::put_item::builders::PutItemFluentBuilder`] to convert
            //! processing results into DynamoDB items.
            //!
            //! If the result is a [`Ok<O, _>`], then a `output` field is added to the item
            //! with a status code of `200`.
            //! If the result is a [`Err<_, CoffeeShopError>`], then an `error` field is added
            //! to the item with the status code of the error. The error message is customised
            //! by the error type of [`CoffeeShopError::ErrorSchema`].
            use super::{ERROR_KEY, OUTPUT_KEY, STATUS_KEY, SUCCESS_KEY, TTL_KEY};
            use crate::{
                helpers, models::{message::ProcessResult, Ticket},
                CoffeeShopError,
            };
            use aws_sdk_dynamodb as dynamodb;
            /// Add items common to both
            /// [`report_ticket_success`](ToItem::report_ticket_success) and
            /// [`report_ticket_failure`](ToItem::report_ticket_failure) to the fluent
            /// builder.
            fn add_common_items(
                builder: dynamodb::operation::put_item::builders::PutItemFluentBuilder,
                partition_key: &str,
                ticket: &Ticket,
                ttl: &tokio::time::Duration,
            ) -> dynamodb::operation::put_item::builders::PutItemFluentBuilder {
                let expiry = chrono::Duration::from_std(*ttl)
                    .map_or_else(
                        |_| chrono::DateTime::<chrono::Utc>::MAX_UTC,
                        |duration| chrono::Utc::now() + duration,
                    );
                builder
                    .item(
                        partition_key,
                        dynamodb::types::AttributeValue::S(ticket.to_owned()),
                    )
                    .item(
                        TTL_KEY,
                        dynamodb::types::AttributeValue::N(
                            expiry.timestamp().to_string(),
                        ),
                    )
            }
            /// Convert a processing result into a DynamoDB item.
            pub trait ToItem: Sized {
                type Output;
                /// Convert the successful processing result into a DynamoDB item.
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn report_ticket_success<
                    'life0,
                    'life1,
                    'life2,
                    'life3,
                    'async_trait,
                    O,
                >(
                    self,
                    partition_key: &'life0 str,
                    ticket: &'life1 Ticket,
                    output: O,
                    ttl: &'life2 tokio::time::Duration,
                    temp_dir: &'life3 tempfile::TempDir,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Self::Output,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    O: 'async_trait + serde::Serialize + Send + Sync,
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    'life2: 'async_trait,
                    'life3: 'async_trait,
                    Self: 'async_trait;
                /// Convert the failed processing result into a DynamoDB item.
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn report_ticket_failure<'life0, 'life1, 'life2, 'async_trait>(
                    self,
                    partition_key: &'life0 str,
                    ticket: &'life1 Ticket,
                    error: CoffeeShopError,
                    ttl: &'life2 tokio::time::Duration,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Self::Output,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    'life2: 'async_trait,
                    Self: 'async_trait;
                /// Convert the processing result into a DynamoDB item.
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn report_ticket_result<'life0, 'life1, 'life2, 'life3, 'async_trait, O>(
                    self,
                    partition_key: &'life0 str,
                    ticket: &'life1 Ticket,
                    result: ProcessResult<O>,
                    ttl: &'life2 tokio::time::Duration,
                    temp_dir: &'life3 tempfile::TempDir,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Self::Output,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    O: serde::Serialize + Send + Sync,
                    O: 'async_trait,
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    'life2: 'async_trait,
                    'life3: 'async_trait,
                    Self: ::core::marker::Send + 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Self::Output,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let result = result;
                        let __ret: Self::Output = {
                            match result {
                                Ok(output) => {
                                    __self
                                        .report_ticket_success(
                                            partition_key,
                                            ticket,
                                            output,
                                            ttl,
                                            temp_dir,
                                        )
                                        .await
                                }
                                Err(error) => {
                                    __self
                                        .report_ticket_failure(partition_key, ticket, error, ttl)
                                        .await
                                }
                            }
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
            impl ToItem
            for dynamodb::operation::put_item::builders::PutItemFluentBuilder {
                type Output = Result<Self, CoffeeShopError>;
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn report_ticket_success<
                    'life0,
                    'life1,
                    'life2,
                    'life3,
                    'async_trait,
                    O,
                >(
                    self,
                    partition_key: &'life0 str,
                    ticket: &'life1 Ticket,
                    output: O,
                    ttl: &'life2 tokio::time::Duration,
                    temp_dir: &'life3 tempfile::TempDir,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Self::Output,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    O: 'async_trait + serde::Serialize + Send + Sync,
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    'life2: 'async_trait,
                    'life3: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Self::Output,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let output = output;
                        let __ret: Self::Output = {
                            let buffer = helpers::serde::serialize(&output, temp_dir)
                                .await?;
                            Ok(
                                add_common_items(__self, partition_key, ticket, ttl)
                                    .item(
                                        STATUS_KEY,
                                        dynamodb::types::AttributeValue::N("200".to_owned()),
                                    )
                                    .item(
                                        SUCCESS_KEY,
                                        dynamodb::types::AttributeValue::Bool(true),
                                    )
                                    .item(
                                        OUTPUT_KEY,
                                        dynamodb::types::AttributeValue::B(
                                            dynamodb::primitives::Blob::new(buffer.read_to_end().await?),
                                        ),
                                    ),
                            )
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn report_ticket_failure<'life0, 'life1, 'life2, 'async_trait>(
                    self,
                    partition_key: &'life0 str,
                    ticket: &'life1 Ticket,
                    error: CoffeeShopError,
                    ttl: &'life2 tokio::time::Duration,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Self::Output,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    'life2: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Self::Output,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let error = error;
                        let __ret: Self::Output = {
                            let error_body = serde_json::to_string(&error.as_json())
                                .expect(
                                    "Failed to serialize the error from the processing result. Please check that the error type is serializable.",
                                );
                            Ok(
                                add_common_items(__self, partition_key, ticket, ttl)
                                    .item(
                                        STATUS_KEY,
                                        dynamodb::types::AttributeValue::N(
                                            error.status_code().as_u16().to_string(),
                                        ),
                                    )
                                    .item(
                                        SUCCESS_KEY,
                                        dynamodb::types::AttributeValue::Bool(false),
                                    )
                                    .item(
                                        ERROR_KEY,
                                        dynamodb::types::AttributeValue::S(error_body),
                                    ),
                            )
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
        pub use process_result_to_item::*;
        mod item_to_process_result {
            use crate::{
                errors::ErrorSchema, helpers,
                models::{message::ProcessResultExport, Ticket},
                CoffeeShopError,
            };
            use axum::http;
            use serde::de::DeserializeOwned;
            use super::{ERROR_KEY, OUTPUT_KEY, STATUS_KEY, SUCCESS_KEY};
            use aws_sdk_dynamodb::types::AttributeValue;
            /// Trait for converting an item to a process result.
            pub trait ToProcessResult {
                /// Attempt to check the status of the item.
                ///
                /// This function do not require the full result to be present in the item;
                /// only the ticket and success status are needed. Only tickets with a success
                /// status will be returned, regardless of whether it is `true` or `false`.
                ///
                /// This does not consume the item; a minimal cloning is performed on the ticket.
                fn to_process_status(
                    &self,
                    partition_key: &str,
                ) -> Result<(Ticket, bool), CoffeeShopError>;
                /// Attempt to convert the item into a process result.
                ///
                /// The return type of this has a nested [`Result`]:
                /// - The outer [`Result<(Ticket, _)`] is the result of the conversion.
                /// - The inner [`ProcessResultExport<O>`] is the actual processing result.
                ///
                /// Only the inner [`ErrorSchema`] is preserved, distinguishing it from a local error.
                /// Not being wrapped in a [`CoffeeShopError::ErrorSchema`], this also
                /// ensure that the error can be [`Clone`]d and serialized into a
                /// [`Response`](axum::http::Response), since the original error could
                /// contain non-serializable types or non-static lifetimes.
                fn to_process_result<O>(
                    self,
                    partition_key: &str,
                ) -> Result<(Ticket, ProcessResultExport<O>), CoffeeShopError>
                where
                    O: DeserializeOwned + Send + Sync;
            }
            impl ToProcessResult for std::collections::HashMap<String, AttributeValue> {
                fn to_process_status(
                    &self,
                    partition_key: &str,
                ) -> Result<(Ticket, bool), CoffeeShopError> {
                    match (self.get(partition_key), self.get(SUCCESS_KEY)) {
                        (
                            Some(AttributeValue::S(ticket)),
                            Some(AttributeValue::Bool(success)),
                        ) => Ok((ticket.clone(), *success)),
                        _ => {
                            Err(
                                CoffeeShopError::DynamoDBMalformedItem(
                                    "A map was retrieved, but its structure could not be parsed."
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                fn to_process_result<O>(
                    mut self,
                    partition_key: &str,
                ) -> Result<(Ticket, ProcessResultExport<O>), CoffeeShopError>
                where
                    O: DeserializeOwned + Send + Sync,
                {
                    match (
                        self.remove(partition_key),
                        self.remove(SUCCESS_KEY),
                        self.remove(STATUS_KEY),
                        self.remove(OUTPUT_KEY),
                        self.remove(ERROR_KEY),
                    ) {
                        (
                            Some(AttributeValue::S(ticket)),
                            Some(AttributeValue::Bool(true)),
                            Some(AttributeValue::N(status)),
                            Some(AttributeValue::B(blob)),
                            None,
                        ) => {
                            let output = helpers::serde::deserialize::<
                                O,
                            >(blob.into_inner())?;
                            {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Info;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "Successfully retrieved processing result for ticket {0}. Status: {1}.",
                                                ticket,
                                                status,
                                            ),
                                            lvl,
                                            &(
                                                "coffeeshop::helpers::dynamodb::item_to_process_result",
                                                "coffeeshop::helpers::dynamodb::item_to_process_result",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            };
                            Ok((ticket, Ok(output)))
                        }
                        (
                            Some(AttributeValue::S(ticket)),
                            Some(AttributeValue::Bool(false)),
                            Some(AttributeValue::N(status)),
                            None,
                            Some(AttributeValue::S(error_json)),
                        ) => {
                            let error: ErrorSchema = serde_json::from_str(&error_json)
                                .inspect_err(|_| {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Error;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Encountered an unparsable error schema for ticket {0}. Status: {1}. Error: {2:?}",
                                                    ticket,
                                                    status,
                                                    error_json,
                                                ),
                                                lvl,
                                                &(
                                                    "coffeeshop::helpers::dynamodb::item_to_process_result",
                                                    "coffeeshop::helpers::dynamodb::item_to_process_result",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                })
                                .unwrap_or_else(|_| ErrorSchema::new(
                                    http::StatusCode::INTERNAL_SERVER_ERROR,
                                    "UnknownProcessingError".to_owned(),
                                    Some(
                                        ::serde_json::Value::Object({
                                            let mut object = ::serde_json::Map::new();
                                            let _ = object
                                                .insert(
                                                    ("message").into(),
                                                    ::serde_json::to_value(
                                                            &"A processing error had occurred, but the error message cannot be parsed; could not report the actual error.",
                                                        )
                                                        .unwrap(),
                                                );
                                            let _ = object
                                                .insert(
                                                    ("original").into(),
                                                    ::serde_json::to_value(&error_json).unwrap(),
                                                );
                                            object
                                        }),
                                    ),
                                ));
                            {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "Successfully retrieved error schema for ticket {0}. Status: {1}. Error: {2:?}",
                                                ticket,
                                                status,
                                                error,
                                            ),
                                            lvl,
                                            &(
                                                "coffeeshop::helpers::dynamodb::item_to_process_result",
                                                "coffeeshop::helpers::dynamodb::item_to_process_result",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            };
                            Ok((ticket, Err(error)))
                        }
                        _ => {
                            Err(
                                CoffeeShopError::DynamoDBMalformedItem(
                                    "A map was retrieved, but its structure could not be parsed."
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
            }
        }
        pub use item_to_process_result::*;
        mod func {
            //! Helper functions to interact with DynamoDB as a key-value result store.
            use aws_sdk_dynamodb as dynamodb;
            use serde::de::DeserializeOwned;
            use std::vec;
            use crate::{
                models::{
                    message::{ProcessResult, ProcessResultExport},
                    Ticket,
                },
                CoffeeShopError,
            };
            use super::*;
            /// Put a processing result into a DynamoDB table.
            pub async fn put_process_result<O>(
                config: &dyn HasDynamoDBConfiguration,
                ticket: &Ticket,
                result: ProcessResult<O>,
                temp_dir: &tempfile::TempDir,
            ) -> Result<(), CoffeeShopError>
            where
                O: serde::Serialize + Send + Sync,
            {
                let client = dynamodb::Client::new(config.aws_config());
                let table = config.dynamodb_table();
                client
                    .put_item()
                    .table_name(table)
                    .report_ticket_result(
                        config.dynamodb_partition_key(),
                        ticket,
                        result,
                        &config.dynamodb_ttl(),
                        temp_dir,
                    )
                    .await?
                    .send()
                    .await
                    .map_err(|sdk_err| {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Failed to put the processing result for ticket {0} into the DynamoDB table {1}. Error: {2:?}",
                                            ticket,
                                            table,
                                            sdk_err,
                                        ),
                                        lvl,
                                        &(
                                            "coffeeshop::helpers::dynamodb::func",
                                            "coffeeshop::helpers::dynamodb::func",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        CoffeeShopError::AWSSdkError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0:?}", sdk_err),
                                );
                                res
                            }),
                        )
                    })
                    .map(|response| {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Successfully put the processing result for ticket {0} into the DynamoDB table {1}. Consumed {2:?} capacity units.",
                                            ticket,
                                            table,
                                            response
                                                .consumed_capacity()
                                                .map(|capacity| capacity.capacity_units())
                                                .unwrap_or_default(),
                                        ),
                                        lvl,
                                        &(
                                            "coffeeshop::helpers::dynamodb::func",
                                            "coffeeshop::helpers::dynamodb::func",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        }
                    })
            }
            /// Get items that matches any given partition keys from a DynamoDB table.
            ///
            /// # Safety
            ///
            /// The [BatchGetItem] only supports up to 100 items per request; this function
            /// does not check the number of tickets. If the number of tickets exceeds 100,
            /// the request will fail.
            ///
            /// [BatchGetItem]: https://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_BatchGetItem.html
            pub async fn get_items_by_tickets_unchecked<C>(
                config: &C,
                tickets: impl Iterator<Item = &Ticket>,
                projection_expression: Option<&[String]>,
            ) -> Result<Vec<DynamoDBItem>, CoffeeShopError>
            where
                C: HasDynamoDBConfiguration,
            {
                let client = dynamodb::Client::new(config.aws_config());
                let table = config.dynamodb_table();
                let keys = tickets
                    .map(|ticket| {
                        let mut map = std::collections::HashMap::new();
                        map.insert(
                            config.dynamodb_partition_key().to_owned(),
                            dynamodb::types::AttributeValue::S(ticket.to_string()),
                        );
                        map
                    })
                    .collect::<Vec<_>>();
                let response = client
                    .batch_get_item()
                    .request_items(
                        config.dynamodb_table(),
                        dynamodb::types::KeysAndAttributes::builder()
                            .set_keys(Some(keys))
                            .set_attributes_to_get(
                                projection_expression.map(|attrs| attrs.to_vec()),
                            )
                            .build()
                            .map_err(|err| {
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Error;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Failed to build the keys and attributes for the batch get item request for table {0}. Error: {1:?}",
                                                    table,
                                                    err,
                                                ),
                                                lvl,
                                                &(
                                                    "coffeeshop::helpers::dynamodb::func",
                                                    "coffeeshop::helpers::dynamodb::func",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                };
                                CoffeeShopError::AWSSdkError(
                                    ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(format_args!("{0:?}", err));
                                        res
                                    }),
                                )
                            })?,
                    )
                    .send()
                    .await
                    .map_err(|sdk_err| CoffeeShopError::AWSSdkError(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("{0:?}", sdk_err),
                            );
                            res
                        }),
                    ))?;
                let consumed_capacity = response
                    .consumed_capacity()
                    .iter()
                    .fold(
                        0.,
                        |acc, capacity| {
                            acc + capacity.capacity_units().unwrap_or_default()
                        },
                    );
                if let Some(mut table_mapper) = response.responses {
                    if let Some(results) = table_mapper.remove(table) {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Retrieved {0} processing results from the DynamoDB table {1}. Consumed {2:?} capacity units.",
                                            results.len(),
                                            table,
                                            consumed_capacity,
                                        ),
                                        lvl,
                                        &(
                                            "coffeeshop::helpers::dynamodb::func",
                                            "coffeeshop::helpers::dynamodb::func",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        return Ok(results);
                    }
                }
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Warn;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!(
                                    "No processing results found for the given tickets in the DynamoDB table {0}. Consumed {1:?} capacity units.",
                                    table,
                                    consumed_capacity,
                                ),
                                lvl,
                                &(
                                    "coffeeshop::helpers::dynamodb::func",
                                    "coffeeshop::helpers::dynamodb::func",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                Ok(::alloc::vec::Vec::new())
            }
            /// Get items that matches any given partition keys from a DynamoDB table.
            pub async fn get_items_by_tickets<C>(
                config: &C,
                tickets: impl ExactSizeIterator<Item = &Ticket>,
                projection_expression: Option<&[String]>,
            ) -> Result<Vec<DynamoDBItem>, CoffeeShopError>
            where
                C: HasDynamoDBConfiguration,
            {
                if tickets.len() == 0 {
                    return Ok(::alloc::vec::Vec::new());
                }
                let chunks_count = (tickets.len() as f32 / 100.).ceil() as usize;
                let chunk_size = (tickets.len() as f32 / chunks_count as f32).ceil()
                    as usize;
                let chunks = {
                    let mut tickets = tickets.collect::<Vec<_>>();
                    let mut chunks = ::alloc::vec::Vec::new();
                    loop {
                        if tickets.len() <= chunk_size {
                            let mut middleman = ::alloc::vec::Vec::new();
                            std::mem::swap(&mut tickets, &mut middleman);
                            chunks.push(middleman);
                            break;
                        } else {
                            chunks.push(tickets.split_off(chunk_size));
                        }
                    }
                    chunks
                };
                futures::future::try_join_all(
                        chunks
                            .into_iter()
                            .map(|chunk| get_items_by_tickets_unchecked::<
                                _,
                            >(config, chunk.into_iter(), projection_expression)),
                    )
                    .await
                    .map(|results| results.into_iter().flatten().collect())
            }
            /// Get the processing results that matches any given partition keys from a DynamoDB table.
            pub async fn get_process_results_by_tickets<O, C>(
                config: &C,
                tickets: impl ExactSizeIterator<Item = &Ticket>,
            ) -> Result<Vec<(Ticket, ProcessResultExport<O>)>, CoffeeShopError>
            where
                O: DeserializeOwned + Send + Sync,
                C: HasDynamoDBConfiguration,
            {
                get_items_by_tickets(config, tickets, None)
                    .await
                    .and_then(|items| {
                        items
                            .into_iter()
                            .map(|item| {
                                item.to_process_result(config.dynamodb_partition_key())
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
            }
            /// Get the statuses that matches any given partition keys from a DynamoDB table.
            pub async fn get_process_successes_by_tickets<C>(
                config: &C,
                tickets: impl ExactSizeIterator<Item = &Ticket>,
            ) -> Result<Vec<(Ticket, bool)>, CoffeeShopError>
            where
                C: HasDynamoDBConfiguration,
            {
                let projection_expression = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        config.dynamodb_partition_key().to_owned(),
                        SUCCESS_KEY.to_owned(),
                    ]),
                );
                get_items_by_tickets(config, tickets, Some(&projection_expression))
                    .await
                    .and_then(|items| {
                        items
                            .into_iter()
                            .map(|item| {
                                item.to_process_status(config.dynamodb_partition_key())
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
            }
            /// Get a single processing result that matches the given partition key from a DynamoDB table.
            /// This function currently is a convenience wrapper around [`get_process_results_by_tickets`];
            /// which could take a bit more computation time than necessary, but reduces the maintenance
            /// overhead of having to maintain two separate functions.
            ///
            /// # Note
            ///
            /// This function is not optimized for performance; it is recommended to use
            /// [`get_process_results_by_tickets`] if you need to get multiple results.
            pub async fn get_process_result_by_ticket<O, C>(
                config: &C,
                ticket: &Ticket,
            ) -> Result<ProcessResultExport<O>, CoffeeShopError>
            where
                O: DeserializeOwned + Send + Sync,
                C: HasDynamoDBConfiguration,
            {
                get_process_results_by_tickets(config, std::iter::once(ticket))
                    .await
                    .and_then(|mut results| {
                        results
                            .pop()
                            .and_then(|(found_ticket, result)| {
                                if found_ticket == *ticket {
                                    Some(result)
                                } else {
                                    {
                                        crate::logger::init();
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "The ticket {0} does not match the found ticket {1} in the DynamoDB table {2}. Reporting as not found; this should not happen.",
                                                        ticket,
                                                        found_ticket,
                                                        config.dynamodb_table(),
                                                    ),
                                                    lvl,
                                                    &(
                                                        "coffeeshop::helpers::dynamodb::func",
                                                        "coffeeshop::helpers::dynamodb::func",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                    };
                                    None
                                }
                            })
                            .ok_or_else(|| {
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Warn;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "No processing result found for the given ticket {0} in the DynamoDB table {1}.",
                                                    ticket,
                                                    config.dynamodb_table(),
                                                ),
                                                lvl,
                                                &(
                                                    "coffeeshop::helpers::dynamodb::func",
                                                    "coffeeshop::helpers::dynamodb::func",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                };
                                CoffeeShopError::ResultNotFound(ticket.to_string())
                            })
                    })
            }
        }
        pub use func::*;
        /// Alias for a DynamoDB item.
        pub type DynamoDBItem = std::collections::HashMap<
            String,
            aws_sdk_dynamodb::types::AttributeValue,
        >;
    }
    pub mod multicast {
        //! Multicast functions and structs for asynchronous communication among [`Shop`](crate::models::Shop) instances within the same cluster.
        pub mod socket {
            //! Unified interface for the creation of sockets, and low-level multicast operations.
            use socket2::{Domain, Protocol, SockAddr, Socket, Type};
            use std::net::{IpAddr, Ipv4Addr, SocketAddr};
            use crate::CoffeeShopError;
            const LOG_TARGET: &str = "coffeeshop::helpers::multicast::socket";
            use super::AsyncSocket;
            /// A helper function to describe a [`SockAddr`].
            ///
            /// This is distinct from [`describe_socket_addr`] which is the [`std::net`] equivalent.
            pub fn describe_sock_addr(sock_addr: &SockAddr) -> String {
                sock_addr
                    .as_socket()
                    .map(|sock_addr| describe_socket_addr(&sock_addr))
                    .unwrap_or_else(|| "(Unknown source)".to_owned())
            }
            /// A helper function to describe a [`SocketAddr`].
            ///
            /// This is distinct from [`describe_sock_addr`] which is the [`socket2`] equivalent.
            pub fn describe_socket_addr(socket_addr: &SocketAddr) -> String {
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}:{1}", socket_addr.ip(), socket_addr.port()),
                    );
                    res
                })
            }
            /// Create a generic UDP socket that can be used for multicast communication.
            ///
            /// The resultant socket can be used for both sending and receiving multicast messages.
            ///
            /// By default, the socket will be:
            /// - non-blocking,
            /// - allow the reuse of the address, and
            /// - bound to the given address.
            pub fn create_udp(
                addr: &SocketAddr,
            ) -> Result<AsyncSocket, CoffeeShopError> {
                let builder = || {
                    let domain = Domain::for_address(*addr);
                    let socket = Socket::new(domain, Type::DGRAM, Some(Protocol::UDP))?;
                    socket.set_nonblocking(true)?;
                    socket.set_reuse_address(true)?;
                    socket.bind(&SockAddr::from(*addr))?;
                    AsyncSocket::new(socket)
                };
                builder().map_err(CoffeeShopError::from_multicast_io_error)
            }
            /// A short hand function to create a UDP socket bound to all IPv4 interfaces.
            ///
            /// This is useful for creating a sender socket.
            pub fn create_udp_all_v4_interfaces(
                port: u16,
            ) -> Result<AsyncSocket, CoffeeShopError> {
                let addr = SocketAddr::from(([0, 0, 0, 0], port));
                create_udp(&addr)
            }
            /// A helper function to set a socket to join a multicast address.
            ///
            /// The resultant socket will listen for multicast messages on all interfaces.
            pub fn join_multicast(
                asocket: &AsyncSocket,
                addr: &SocketAddr,
            ) -> Result<(), CoffeeShopError> {
                let socket = asocket.get_ref();
                let ip_addr = addr.ip();
                if !ip_addr.is_multicast() {
                    return Err(CoffeeShopError::InvalidConfiguration {
                        field: "multicast_host",
                        message: ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Address {0} is not a multicast address",
                                    ip_addr,
                                ),
                            );
                            res
                        }),
                    });
                }
                match ip_addr {
                    IpAddr::V4(ref mdns_v4) => {
                        socket.join_multicast_v4(mdns_v4, &Ipv4Addr::new(0, 0, 0, 0))
                    }
                    IpAddr::V6(ref mdns_v6) => {
                        socket
                            .join_multicast_v6(mdns_v6, 0)
                            .and_then(|_| socket.set_only_v6(true))
                    }
                }
                    .map_err(CoffeeShopError::from_multicast_io_error)
            }
            /// A helper function to send a multicast message.
            pub async fn send_multicast(
                asocket: &AsyncSocket,
                addr: &SocketAddr,
                data: &[u8],
            ) -> Result<usize, CoffeeShopError> {
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!(
                                    "Sending {0} bytes to {1:?}...",
                                    data.len(),
                                    describe_sock_addr(&SockAddr::from(*addr)),
                                ),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::helpers::multicast::socket",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                asocket
                    .write(|socket| socket.send_to(data, &SockAddr::from(*addr)))
                    .await
                    .map_err(CoffeeShopError::from_multicast_io_error)
            }
            /// A helper function to receive a multicast message.
            pub async fn receive_multicast(
                asocket: &AsyncSocket,
                buffer_size: usize,
            ) -> Result<(Vec<u8>, SockAddr), CoffeeShopError> {
                let mut inner_buffer = ::alloc::vec::from_elem(
                    core::mem::MaybeUninit::uninit(),
                    buffer_size,
                );
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("Waiting for message..."),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::helpers::multicast::socket",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                let result = asocket
                    .read(|socket| socket.recv_from(&mut inner_buffer))
                    .await;
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Debug;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("Received message."),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::helpers::multicast::socket",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                result
                    .map(|(size, addr)| {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Debug;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Received {0} bytes from {1:?}.",
                                            size,
                                            describe_sock_addr(&addr),
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::helpers::multicast::socket",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        (
                            (0..size)
                                .map(|i| unsafe { inner_buffer[i].assume_init() })
                                .collect::<Vec<_>>(),
                            addr,
                        )
                    })
                    .map_err(CoffeeShopError::from_multicast_io_error)
            }
        }
        /// The async socket type used in this crate.
        pub use tokio_socket2::TokioSocket2 as AsyncSocket;
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
        use crate::{models::message::ProcessResult, CoffeeShopError};
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
        ) -> ProcessResult<O> {
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
        mod config {
            use crate::helpers::aws::{self, HasAWSSdkConfig};
            /// A [`HasSQSConfiguration`] contains the configuration for the DynamoDB table
            /// that the [`Shop`] will be using.
            pub trait HasSQSConfiguration: HasAWSSdkConfig {
                /// The name of the SQS table.
                fn sqs_queue_url(&self) -> &str;
                /// Extract the configuration as a separate struct.
                ///
                /// This is useful if the main configuration struct is too large, or it
                /// lacks certain traits such as [`Send`] or [`Sync`].
                fn sqs_configuration(&self) -> SQSConfiguration {
                    SQSConfiguration {
                        queue_url: self.sqs_queue_url().to_owned(),
                        aws_config: self.aws_config().clone(),
                    }
                }
            }
            /// A minimal implementation of [`SQSConfiguration`] for testing purposes, or
            /// to use this module without a full [`Shop`] configuration.
            pub struct SQSConfiguration {
                pub queue_url: String,
                pub aws_config: aws::SdkConfig,
            }
            impl HasAWSSdkConfig for SQSConfiguration {
                fn aws_config(&self) -> &aws::SdkConfig {
                    &self.aws_config
                }
            }
            impl HasSQSConfiguration for SQSConfiguration {
                fn sqs_queue_url(&self) -> &str {
                    &self.queue_url
                }
            }
        }
        pub use config::*;
        pub mod encoding {
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
            pub const BASE64_ENCODER: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD_NO_PAD;
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
        }
        mod func {
            use crate::{
                helpers, models::{message, Ticket},
                CoffeeShopError,
            };
            use aws_sdk_sqs as sqs;
            use super::{encoding, HasSQSConfiguration, StagedReceipt};
            /// Put a ticket into the AWS SQS queue.
            pub async fn put_ticket<Q, I>(
                config: &dyn HasSQSConfiguration,
                input: message::CombinedInput<Q, I>,
                temp_dir: &tempfile::TempDir,
            ) -> Result<Ticket, CoffeeShopError>
            where
                Q: message::QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                let client = sqs::Client::new(config.aws_config());
                let serialized_input = helpers::serde::serialize(&input, temp_dir)
                    .await?;
                let response = client
                    .send_message()
                    .queue_url(config.sqs_queue_url())
                    .message_body(
                        encoding::encode(&serialized_input.read_to_end().await?).await?,
                    )
                    .send()
                    .await
                    .map_err(|sdk_err| {
                        CoffeeShopError::from_aws_sqs_send_message_error(
                            sdk_err.into_service_error(),
                        )
                    })?;
                response
                    .message_id()
                    .map(Ticket::from)
                    .ok_or_else(|| {
                        CoffeeShopError::UnexpectedAWSResponse(
                            "No message ID returned upon sending message.".to_string(),
                        )
                    })
            }
            /// Retrieve a ticket from the AWS SQS queue.
            pub async fn retrieve_ticket<Q, I>(
                config: &dyn HasSQSConfiguration,
                timeout: Option<tokio::time::Duration>,
            ) -> Result<StagedReceipt<Q, I>, CoffeeShopError>
            where
                Q: message::QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                StagedReceipt::receive(config, timeout).await
            }
            /// Purge a queue of all messages.
            pub async fn purge_tickets(
                config: &dyn HasSQSConfiguration,
            ) -> Result<(), CoffeeShopError> {
                let client = sqs::Client::new(config.aws_config());
                client
                    .purge_queue()
                    .queue_url(config.sqs_queue_url())
                    .send()
                    .await
                    .map_err(|err| CoffeeShopError::AWSSdkError(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0:?}", err));
                            res
                        }),
                    ))?;
                Ok(())
            }
            /// Get ticket count.
            pub async fn get_ticket_count(
                config: &dyn HasSQSConfiguration,
            ) -> Result<usize, CoffeeShopError> {
                let client = sqs::Client::new(config.aws_config());
                let response = client
                    .get_queue_attributes()
                    .queue_url(config.sqs_queue_url())
                    .attribute_names(
                        sqs::types::QueueAttributeName::ApproximateNumberOfMessages,
                    )
                    .send()
                    .await
                    .map_err(|err| CoffeeShopError::AWSSdkError(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0:?}", err));
                            res
                        }),
                    ))?;
                response
                    .attributes
                    .ok_or_else(|| CoffeeShopError::UnexpectedAWSResponse(
                        "Missing attributes".to_string(),
                    ))
                    .and_then(|attributes| {
                        attributes
                            .get(
                                &sqs::types::QueueAttributeName::ApproximateNumberOfMessages,
                            )
                            .ok_or_else(|| {
                                CoffeeShopError::UnexpectedAWSResponse(
                                    "Missing approximate number of messages".to_string(),
                                )
                            })
                            .and_then(|count| {
                                count
                                    .parse::<usize>()
                                    .map_err(|err| {
                                        CoffeeShopError::UnexpectedAWSResponse(
                                            ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!(
                                                        "Failed to parse the approximate number of messages {0:?}: {1}",
                                                        count,
                                                        err,
                                                    ),
                                                );
                                                res
                                            }),
                                        )
                                    })
                            })
                    })
            }
        }
        pub use func::*;
        mod staged_receipt {
            use aws_sdk_sqs as sqs;
            use std::sync::OnceLock;
            use crate::{
                helpers::{serde::deserialize, sqs::HasSQSConfiguration},
                models::{message, Ticket},
                CoffeeShopError,
            };
            use super::encoding;
            const LOG_TARGET: &str = "coffeeshop::helpers::sqs::staged_receipt";
            /// The default wait time for receiving messages from SQS.
            ///
            /// When there is no message in the queue, the [`Barista`]s will wait for this
            /// duration before logging a message, and then checking the queue again.
            const DEFAULT_WAIT_TIME: tokio::time::Duration = tokio::time::Duration::from_secs(
                20,
            );
            /// A received message from SQS that is staged for processing, before
            /// a reply to SQS had been sent on deleting the message or its visibility
            /// changed back to visible.
            pub struct StagedReceipt<Q, I>
            where
                Q: message::QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                client: sqs::Client,
                pub ticket: Ticket,
                message: message::CombinedInput<Q, I>,
                pub receipt_handle: String,
                pub queue_url: String,
                /// Completed
                completed: OnceLock<bool>,
            }
            impl<Q, I> StagedReceipt<Q, I>
            where
                Q: message::QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                /// Create a new [`StagedReceipt`] instance.
                pub async fn receive(
                    config: &dyn HasSQSConfiguration,
                    timeout: Option<tokio::time::Duration>,
                ) -> Result<Self, CoffeeShopError> {
                    let client = sqs::Client::new(config.aws_config());
                    let timeout = timeout
                        .unwrap_or(DEFAULT_WAIT_TIME)
                        .min(tokio::time::Duration::from_secs(20));
                    let receive_results = client
                        .receive_message()
                        .queue_url(config.sqs_queue_url())
                        .max_number_of_messages(1)
                        .wait_time_seconds(timeout.as_secs() as i32)
                        .send()
                        .await
                        .map_err(|err| CoffeeShopError::AWSSdkError(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(format_args!("{0:?}", err));
                                res
                            }),
                        ))?;
                    let message = receive_results
                        .messages
                        .and_then(|mut messages| messages.pop());
                    if let Some(message) = message {
                        let receipt_handle = message
                            .receipt_handle
                            .ok_or_else(|| {
                                CoffeeShopError::UnexpectedAWSResponse(
                                    "Missing SQS receipt handle".to_string(),
                                )
                            })?;
                        let body = message
                            .body
                            .ok_or_else(|| {
                                CoffeeShopError::UnexpectedAWSResponse(
                                    "Missing SQS message body".to_string(),
                                )
                            })?;
                        let ticket = message
                            .message_id
                            .ok_or_else(|| {
                                CoffeeShopError::UnexpectedAWSResponse(
                                    "Missing SQS message ID".to_string(),
                                )
                            })?;
                        let message = deserialize(encoding::decode(&body).await?)
                            .inspect_err(|err| {
                                if let CoffeeShopError::ResultBinaryConversionError(_) = err {
                                    {
                                        crate::logger::init();
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "Failed to deserialize the message body of ticket {0} from queue {1}. This can be caused by Is the SQS queue exclusively used by this app?",
                                                        ticket,
                                                        config.sqs_queue_url(),
                                                    ),
                                                    lvl,
                                                    &(
                                                        LOG_TARGET,
                                                        "coffeeshop::helpers::sqs::staged_receipt",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                    }
                                }
                            })?;
                        Ok(Self {
                            client,
                            ticket,
                            message,
                            receipt_handle,
                            queue_url: config.sqs_queue_url().to_owned(),
                            completed: OnceLock::new(),
                        })
                    } else {
                        Err(CoffeeShopError::AWSSQSQueueEmpty(timeout))
                    }
                }
                /// Get the query from the message.
                pub fn query(&self) -> &Q {
                    &self.message.query
                }
                /// Get the input from the message.
                pub fn input(&self) -> Option<&I> {
                    self.message.input.as_ref()
                }
                /// Mark the message as completed.
                pub async fn complete(
                    self,
                    result: bool,
                ) -> Result<(), CoffeeShopError> {
                    self.completed
                        .set(result)
                        .map_err(|_| {
                            CoffeeShopError::AWSSQSStagedReceiptAlreadyCompleted(
                                if result { "deleted" } else { "aborted" },
                            )
                        })?;
                    if result {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Completed message processing for ticket {0}, deleting it from the queue.",
                                            self.ticket,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::helpers::sqs::staged_receipt",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        self.client
                            .delete_message()
                            .queue_url(&self.queue_url)
                            .receipt_handle(&self.receipt_handle)
                            .send()
                            .await
                            .map_err(|err| CoffeeShopError::AWSSdkError(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(format_args!("{0:?}", err));
                                    res
                                }),
                            ))
                            .map(|_output| ())
                    } else {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Warn;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Aborting message processing for ticket {0}, returning it to the queue.",
                                            self.ticket,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::helpers::sqs::staged_receipt",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        self.client
                            .change_message_visibility()
                            .queue_url(&self.queue_url)
                            .receipt_handle(&self.receipt_handle)
                            .visibility_timeout(0)
                            .send()
                            .await
                            .map_err(|err| CoffeeShopError::AWSSdkError(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(format_args!("{0:?}", err));
                                    res
                                }),
                            ))
                            .map(|_output| ())
                    }
                }
                /// Abort the message processing.
                pub async fn abort(self) -> Result<(), CoffeeShopError> {
                    self.complete(false).await
                }
                /// Delete the message from the queue.
                pub async fn delete(self) -> Result<(), CoffeeShopError> {
                    self.complete(true).await
                }
            }
            impl<Q, I> Drop for StagedReceipt<Q, I>
            where
                Q: message::QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                /// Drop the [`StagedReceipt`] instance.
                ///
                /// If the message was not completed, log an error.
                /// If the `sqs_strict` feature is enabled, panic.
                ///
                /// # Note
                ///
                /// We could not use the `Drop` trait to delete the message from the queue
                /// due to the asynchronous nature of the `async fn complete` method.
                fn drop(&mut self) {
                    if self.completed.get().is_none() {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Staged receipt for ticket {0} was dropped without being completed.",
                                            self.ticket,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::helpers::sqs::staged_receipt",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        if false {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "Staged receipt for ticket {0} was dropped without being completed; please ensure you used `StagedReceipt::delete` or `StagedReceipt::abort` to complete the message.",
                                        self.ticket,
                                    ),
                                );
                            };
                        }
                    }
                }
            }
        }
        pub use staged_receipt::*;
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
    //! - [`Order`]: The struct that contains the processed ticket and the waiter
    //!   notification.
    //! - [`message`]: The module that contains the request and response structs for
    //!   internal communication.
    mod announcer {
        use super::{message, Machine, Shop};
        use prost::Message;
        use serde::{de::DeserializeOwned, Serialize};
        use socket2::SockAddr;
        use std::sync::{Arc, OnceLock, Weak};
        use tokio::sync::Notify;
        use crate::{helpers::multicast, CoffeeShopError};
        /// The default buffer size for receiving multicast messages.
        const DEFAULT_BUFFER_SIZE: usize = 1024;
        const LOG_TARGET: &str = "coffeeshop::models::announcer";
        /// An [`Announcer`] is a person who broadcasts the orders that are ready to other
        /// [`Announcer`]s in other [`Shop`]s.
        pub struct Announcer<Q, I, O, F>
        where
            Q: message::QueryType,
            I: Serialize + DeserializeOwned + Send + Sync,
            O: Serialize + DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            shop: Weak<Shop<Q, I, O, F>>,
            sender: OnceLock<multicast::AsyncSocket>,
            receiver: OnceLock<multicast::AsyncSocket>,
        }
        impl<Q, I, O, F> std::fmt::Debug for Announcer<Q, I, O, F>
        where
            Q: message::QueryType,
            I: Serialize + DeserializeOwned + Send + Sync,
            O: Serialize + DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Announcer").field("shop", &self.shop).finish()
            }
        }
        impl<Q, I, O, F> Announcer<Q, I, O, F>
        where
            Q: message::QueryType,
            I: Serialize + DeserializeOwned + Send + Sync,
            O: Serialize + DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            /// Create a new, uninitialized announcer with the given shop.
            ///
            /// We could not complete the initialization in the constructor because
            /// the shop provided here is a cyclical reference that would not have been
            /// created yet, so none of the multicast parameters would have been
            /// available.
            ///
            /// Call [`init`](Self::init) after [`Shop`] is initialized to complete the
            /// initialization; this step is typically done in the [`Shop::new`] constructor.
            pub fn new(shop: Weak<Shop<Q, I, O, F>>) -> Self {
                Self {
                    shop,
                    sender: OnceLock::new(),
                    receiver: OnceLock::new(),
                }
            }
            /// Get the multicast address that this announcer is serving.
            fn multicast_addr(&self) -> std::net::SocketAddr {
                self.shop().config.multicast_addr()
            }
            /// Initialize the multicast socket for sending.
            ///
            /// The socket for sending is bound to all IPv4 interfaces, and is not active
            /// until a message is sent. It is stateless; hence we only need one for each
            /// [`Announcer`] instance.
            fn init_sender(&self) -> Result<multicast::AsyncSocket, CoffeeShopError> {
                let addr = self.multicast_addr();
                multicast::socket::create_udp_all_v4_interfaces(0)
                    .inspect_err(|err| {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Failed to create multicast sender socket at {0:?}: {1}",
                                            &addr,
                                            err,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::models::announcer",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        }
                    })
            }
            /// Initialize the multicast socket for listening.
            ///
            /// # Note
            ///
            /// In the future, once [once_cell_try](https://github.com/rust-lang/rust/issues/109737)
            /// is stabilized, then this function can have a return type of `Result`.
            fn init_receiver(&self) -> Result<multicast::AsyncSocket, CoffeeShopError> {
                let addr = self.multicast_addr();
                multicast::socket::create_udp(&addr)
                    .inspect_err(|err| {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Failed to create multicast socket at {0:?}: {1}",
                                            &addr,
                                            err,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::models::announcer",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        }
                    })
                    .and_then(|asocket| {
                        multicast::socket::join_multicast(&asocket, &addr)
                            .inspect_err(|err| {
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Error;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Failed to join multicast group at {0:?}: {1}",
                                                    &addr,
                                                    err,
                                                ),
                                                lvl,
                                                &(
                                                    LOG_TARGET,
                                                    "coffeeshop::models::announcer",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                }
                            })
                            .map(|_| asocket)
                    })
            }
            /// Initialize the [`Announcer`].
            ///
            /// Compared to not initializing the [`Announcer`] and using it directly, this
            /// provides each of the initialization steps a chance to fail, and the error can
            /// be gracefully handled.
            ///
            /// Using this method straight after [`Shop`] is initialized is strongly
            /// recommended. This should be done for you in the [`Shop::new`] constructor.
            ///
            /// # Safety
            ///
            /// This method is safe to call only if no other references to the [`Announcer`]
            /// exist; this is due to a time-of-check-time-of-use (TOCTOU) situation where
            /// the sender and receiver may be initialized by another thread between the
            /// check and the initialization.
            ///
            /// In such a case, this method may return an error about Multicast Sockets not
            /// being initialized. In the case where double initialization may have occurred,
            /// this should not be a problem, as the second initialization will be a no-op; but
            /// it is still recommended to avoid this situation.
            pub fn init(&self) -> Result<(), CoffeeShopError> {
                if self.sender.get().is_none() {
                    drop(self.sender.set(self.init_sender()?));
                }
                if self.receiver.get().is_none() {
                    drop(self.receiver.set(self.init_receiver()?));
                }
                Ok(())
            }
            /// Get the back reference to the shop that this announcer is serving.
            pub fn shop(&self) -> Arc<Shop<Q, I, O, F>> {
                self.shop
                    .upgrade()
                    .expect(
                        "Shop has been dropped; this should not be possible in normal use. Please report this to the maintainer.",
                    )
            }
            /// Get the multicast socket for sending.
            pub fn sender(&self) -> &multicast::AsyncSocket {
                self.sender
                    .get_or_init(|| {
                        self.init_sender()
                            .expect("Failed to initialize multicast sender.")
                    })
            }
            /// Get the multicast socket for receiving.
            pub fn receiver(&self) -> &multicast::AsyncSocket {
                self.receiver
                    .get_or_init(|| {
                        self.init_receiver()
                            .expect("Failed to initialize multicast receiver.")
                    })
            }
            /// Send a message to all announcers in the multicast group.
            ///
            /// This function will encode the message and send it to the multicast group.
            ///
            /// # Returns
            ///
            /// The number of bytes sent.
            pub async fn send_message(
                &self,
                msg: message::MulticastMessage,
            ) -> Result<usize, CoffeeShopError> {
                let encoded = msg.encode_to_vec();
                multicast::socket::send_multicast(
                        self.sender(),
                        &self.multicast_addr(),
                        &encoded,
                    )
                    .await
                    .inspect_err(|err| {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!("Failed to send multicast message: {0}", err),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::models::announcer",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        }
                    })
            }
            /// Static method to transform a received message into a [`MulticastMessage`].
            fn transform_message(
                &self,
                data: Vec<u8>,
                addr: SockAddr,
            ) -> Result<message::MulticastMessage, CoffeeShopError> {
                message::MulticastMessage::decode(&data[..])
                    .inspect_err(|err| {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Failed to decode multicast message from {0:?}: {1}",
                                            &addr,
                                            err,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::models::announcer",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        }
                    })
                    .map_err(|err| CoffeeShopError::InvalidMulticastMessage {
                        data,
                        addr: multicast::socket::describe_sock_addr(&addr),
                        error: err,
                    })
            }
            /// Receive a message from the multicast group, transform it into a [`MulticastMessage`],
            /// and handle it according to its message type.
            ///
            /// Internal function, meant to be called by the [`listen_for_announcements`] function.
            async fn received_message_handler(
                &self,
                data: Vec<u8>,
                addr: SockAddr,
            ) -> Result<(), CoffeeShopError> {
                let message = self.transform_message(data, addr)?;
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("Received multicast message: {0:?}", &message),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::models::announcer",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                match (message.kind(), message.status()) {
                    (
                        message::MulticastMessageKind::Ticket,
                        status,
                    ) if status.is_finished() => {
                        let shop = self.shop();
                        if let Some(order) = shop.get_order(&message.ticket).await {
                            order
                                .complete(
                                    status == message::MulticastMessageStatus::Complete,
                                )
                                .inspect_err(|err| {
                                    {
                                        crate::logger::init();
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "Failed to set order {0:?} to complete, ignoring: {1}",
                                                        &message.ticket,
                                                        err,
                                                    ),
                                                    lvl,
                                                    &(
                                                        LOG_TARGET,
                                                        "coffeeshop::models::announcer",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                    }
                                })?;
                        } else {
                            {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Info;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "Received completion message for irrelevant ticket {0:?}, ignoring.",
                                                &message.ticket,
                                            ),
                                            lvl,
                                            &(
                                                LOG_TARGET,
                                                "coffeeshop::models::announcer",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            }
                        }
                    }
                    (kind, status) => {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Received irrelevant multicast message kind and status, ignored: {0:?}, {1:?}",
                                            kind,
                                            status,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::models::announcer",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                    }
                }
                Ok(())
            }
            /// Listen for announcements from other [`Announcer`]s as well as itself.
            pub async fn listen_for_announcements(
                &self,
                shutdown_signal: Arc<Notify>,
            ) -> Result<(), CoffeeShopError> {
                let mut message_count: u64 = 0;
                let task = async {
                    loop {
                        if let Ok((data, addr)) = multicast::socket::receive_multicast(
                                self.receiver(),
                                DEFAULT_BUFFER_SIZE,
                            )
                            .await
                            .inspect_err(|err| {
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Error;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Failed to receive multicast message, skipping: {0}",
                                                    err,
                                                ),
                                                lvl,
                                                &(
                                                    LOG_TARGET,
                                                    "coffeeshop::models::announcer",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                }
                            })
                        {
                            if self.received_message_handler(data, addr).await.is_ok() {
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Info;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Processed multicast message #{0} successfully.",
                                                    message_count,
                                                ),
                                                lvl,
                                                &(
                                                    LOG_TARGET,
                                                    "coffeeshop::models::announcer",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                };
                            } else {
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Error;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Failed to process multicast message #{0}, skipping.",
                                                    message_count,
                                                ),
                                                lvl,
                                                &(
                                                    LOG_TARGET,
                                                    "coffeeshop::models::announcer",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                }
                            }
                        } else {
                            {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "Failed to receive multicast message, skipping.",
                                            ),
                                            lvl,
                                            &(
                                                LOG_TARGET,
                                                "coffeeshop::models::announcer",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            }
                        }
                        message_count = message_count.wrapping_add(1);
                    }
                };
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
                        let futures_init = (shutdown_signal.notified(), task);
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
                            {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "Received shutdown signal, terminating announcer.",
                                            ),
                                            lvl,
                                            &(
                                                LOG_TARGET,
                                                "coffeeshop::models::announcer",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            };
                            Ok(())
                        }
                        __tokio_select_util::Out::_1(result) => {
                            {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "Failed to listen for announcements: {0:?}",
                                                result,
                                            ),
                                            lvl,
                                            &(
                                                LOG_TARGET,
                                                "coffeeshop::models::announcer",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            };
                            result
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
            }
        }
    }
    pub use announcer::Announcer;
    mod barista {
        use serde::{de::DeserializeOwned, Serialize};
        use std::{ops::Deref, sync::{atomic::AtomicUsize, Arc, Weak}};
        use tokio::sync::Notify;
        use super::{
            message::{self, MulticastMessage, ProcessResult},
            Machine, Shop,
        };
        use crate::{helpers, models::message::MulticastMessageStatus, CoffeeShopError};
        const LOG_TARGET: &str = "coffeeshop::models::barista";
        const BARISTA_REPORT_IDLE: tokio::time::Duration = tokio::time::Duration::from_secs(
            20,
        );
        /// A [`Barista`] instance that acts as a worker for the shop.
        ///
        /// A shop can have any positive number of [`Barista`] instances; they are responsible
        /// for taking [`Ticket`]s from the SQS queue, process them, and send the results to
        /// DynamoDB with the [`Ticket`] being the key.
        ///
        /// They are also responsible for sending a multicast message to all the waiters in
        /// the same cluster (including those in different [`Shop`]s), so that the waiters can
        /// retrieve the results when ready instead of polling the DynamoDB table.
        pub struct Barista<Q, I, O, F>
        where
            Q: message::QueryType,
            I: Serialize + DeserializeOwned + Send + Sync,
            O: Serialize + DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            /// A back reference to the shop that this barista is serving.
            pub shop: Weak<Shop<Q, I, O, F>>,
            /// The total amount of historical requests processed.
            pub process_count: AtomicUsize,
        }
        #[automatically_derived]
        impl<
            Q: ::core::fmt::Debug,
            I: ::core::fmt::Debug,
            O: ::core::fmt::Debug,
            F: ::core::fmt::Debug,
        > ::core::fmt::Debug for Barista<Q, I, O, F>
        where
            Q: message::QueryType,
            I: Serialize + DeserializeOwned + Send + Sync,
            O: Serialize + DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Barista",
                    "shop",
                    &self.shop,
                    "process_count",
                    &&self.process_count,
                )
            }
        }
        impl<Q, I, O, F> Barista<Q, I, O, F>
        where
            Q: message::QueryType,
            I: Serialize + DeserializeOwned + Send + Sync,
            O: Serialize + DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            /// Create a new [`Barista`] instance.
            pub fn new(shop: Weak<Shop<Q, I, O, F>>) -> Self {
                Self {
                    shop,
                    process_count: AtomicUsize::new(0),
                }
            }
            /// Get the back reference to the shop that this barista is serving.
            pub fn shop(&self) -> Arc<Shop<Q, I, O, F>> {
                self.shop
                    .upgrade()
                    .expect(
                        "Shop has been dropped; this should not be possible in normal use. Please report this to the maintainer.",
                    )
            }
            /// Get the total amount of historical requests processed.
            pub fn get_process_count(&self) -> usize {
                self.process_count.load(std::sync::atomic::Ordering::Relaxed)
            }
            /// Ask the [`Barista`] to start serving.
            ///
            /// This function never returns, and will loop indefinitely until the
            /// program is terminated.
            pub async fn serve(
                &self,
                shutdown_signal: Arc<Notify>,
            ) -> Result<(), CoffeeShopError> {
                let task = async {
                    loop {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!("A Barista is waiting for the next ticket..."),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::models::barista",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        match self.process_next_ticket(Some(BARISTA_REPORT_IDLE)).await {
                            Ok(_) => {}
                            Err(crate::CoffeeShopError::AWSSQSQueueEmpty(duration)) => {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Info;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "No tickets in the queue after {0:?}; trying again.",
                                                duration,
                                            ),
                                            lvl,
                                            &(
                                                LOG_TARGET,
                                                "coffeeshop::models::barista",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            }
                            Err(err) => {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!("Error processing ticket: {0}", err),
                                            lvl,
                                            &(
                                                LOG_TARGET,
                                                "coffeeshop::models::barista",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            }
                        }
                    }
                };
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
                        let futures_init = (shutdown_signal.notified(), task);
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
                            {
                                crate::logger::init();
                                {
                                    let lvl = ::log::Level::Warn;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "Received shutdown signal, terminating announcer.",
                                            ),
                                            lvl,
                                            &(
                                                LOG_TARGET,
                                                "coffeeshop::models::barista",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                            };
                            Ok(())
                        }
                        __tokio_select_util::Out::_1(_) => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("The barista task should never return."),
                                ),
                            );
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
            }
            /// Serve all the baristas in the list.
            pub async fn serve_all(
                baristas: &[Self],
                shutdown_signal: Arc<Notify>,
            ) -> Result<(), CoffeeShopError> {
                let tasks = baristas
                    .iter()
                    .map(|barista| { barista.serve(shutdown_signal.clone()) });
                futures::future::try_join_all(tasks).await.map(|_| ())
            }
            /// Process a ticket from the SQS queue.
            pub async fn process_ticket(
                &self,
                receipt: &helpers::sqs::StagedReceipt<Q, I>,
            ) -> ProcessResult<O> {
                self.process_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                self.shop()
                    .coffee_machine
                    .call(receipt.query(), receipt.input())
                    .await
                    .map_err(CoffeeShopError::ProcessingError)
            }
            /// Fetch the next ticket from the SQS queue, process it, and send the result to DynamoDB.
            #[allow(unused_variables)]
            pub async fn process_next_ticket(
                &self,
                timeout: Option<tokio::time::Duration>,
            ) -> Result<(), crate::CoffeeShopError> {
                let shop = self.shop();
                let receipt: helpers::sqs::StagedReceipt<Q, I> = helpers::sqs::retrieve_ticket(
                        shop.deref(),
                        timeout,
                    )
                    .await?;
                let result = async {
                    let process_result = self.process_ticket(&receipt).await;
                    let status = if process_result.is_ok() {
                        MulticastMessageStatus::Complete
                    } else {
                        MulticastMessageStatus::Rejected
                    };
                    helpers::dynamodb::put_process_result(
                            shop.deref(),
                            &receipt.ticket,
                            process_result,
                            &shop.temp_dir,
                        )
                        .await?;
                    {
                        crate::logger::init();
                        {
                            let lvl = ::log::Level::Info;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!(
                                        "Successfully processed ticket {0}.",
                                        &receipt.ticket,
                                    ),
                                    lvl,
                                    &(
                                        LOG_TARGET,
                                        "coffeeshop::models::barista",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        };
                    };
                    Ok::<_, CoffeeShopError>(status)
                }
                    .await;
                let ticket = receipt.ticket.clone();
                let status = if let Ok(status) = result {
                    status
                } else {
                    MulticastMessageStatus::Failure
                };
                self.shop()
                    .announcer
                    .send_message(
                        MulticastMessage::new(
                            &self.shop().name,
                            &receipt.ticket,
                            message::MulticastMessageKind::Ticket,
                            status,
                        ),
                    )
                    .await
                    .unwrap_or_else(|err| {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Failed to send multicast message for ticket {0}, ignoring. We\'ll let the collection point discover the result itself: {1}",
                                            &ticket,
                                            err,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::models::barista",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        0
                    });
                if result.is_ok() {
                    receipt.delete().await?;
                } else {
                    receipt.abort().await?;
                }
                result.map(|_| ())
            }
        }
    }
    pub use barista::Barista;
    mod order {
        use std::sync::Arc;
        use hashbrown::HashMap;
        use serde::de::DeserializeOwned;
        use crate::{helpers, models::Ticket, CoffeeShopError};
        use super::message::ProcessResultExport;
        /// The log target for this module.
        const LOG_TARGET: &str = "coffeeshop::models::order";
        /// A collection of [`Order`]s that are being processed.
        pub type Orders = HashMap<String, Arc<Order>>;
        /// A [`Delivery`] is a structure that contains:
        /// - [`OnceLock`](std::sync::OnceLock) which will be populated with the processed ticket
        ///   once it is ready, and
        /// - [`Notify`](tokio::sync::Notify) instance to notify the [`Waiter`] that the ticket is ready.
        ///
        /// The collection point will [push the result](Delivery::complete) into the [`Delivery::result`]
        /// and notify all the interested parties when the ticket is ready.
        pub struct Order {
            ticket: Ticket,
            /// The processed ticket result.
            pub result: std::sync::OnceLock<(tokio::time::Instant, bool)>,
            /// A [`Notify`](tokio::sync::Notify) instance to notify the waiter that the ticket is ready.
            notify: tokio::sync::Notify,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Order {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Order",
                    "ticket",
                    &self.ticket,
                    "result",
                    &self.result,
                    "notify",
                    &&self.notify,
                )
            }
        }
        impl Order {
            /// Create a new [`Delivery`] instance.
            pub fn new(ticket: Ticket) -> Self {
                Self {
                    ticket,
                    result: std::sync::OnceLock::new(),
                    notify: tokio::sync::Notify::new(),
                }
            }
            /// Get the result of the ticket if one is available.
            pub fn result(&self) -> Option<&(tokio::time::Instant, bool)> {
                self.result.get()
            }
            /// Get the age of the result.
            pub fn age_of_result(&self) -> Option<tokio::time::Duration> {
                self.result().map(|(instant, _)| instant.elapsed())
            }
            /// Complete the ticket with the result and the timestamp.
            pub fn complete_with_timestamp(
                &self,
                success: bool,
                timestamp: tokio::time::Instant,
            ) -> Result<(), CoffeeShopError> {
                self.result
                    .set((timestamp, success))
                    .map_err(|_| CoffeeShopError::ResultAlreadySet)?;
                self.notify.notify_waiters();
                Ok(())
            }
            /// Notify the waiter that the ticket is ready.
            pub fn complete(&self, success: bool) -> Result<(), CoffeeShopError> {
                self.complete_with_timestamp(success, tokio::time::Instant::now())
            }
            /// Check if this result is fulfilled.
            pub fn is_fulfilled(&self) -> bool {
                self.result().is_some()
            }
            /// Check if this result is stale.
            ///
            /// A result is considered stale if it has a result set for more than a certain timeout,
            /// but no waiters are waiting for it.
            ///
            /// This method can only be used on [`Arc<Order>`] instances; which is typically
            /// used in conjunction with [`Orders::get`].
            pub fn is_stale(self: &Arc<Self>, max_age: std::time::Duration) -> bool {
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!(
                                    "Order has {0} strong references and the result is {1} seconds old.",
                                    Arc::strong_count(self),
                                    self
                                        .age_of_result()
                                        .map(|age| age.as_secs_f32())
                                        .unwrap_or(0.),
                                ),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::models::order",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                match (Arc::strong_count(self), self.age_of_result()) {
                    (n, Some(age)) if n <= 1 && age > max_age => true,
                    _ => false,
                }
            }
            /// Attempt to fetch the process result from the DynamoDB.
            pub async fn fetch<O, C>(
                &self,
                config: &C,
            ) -> Result<ProcessResultExport<O>, CoffeeShopError>
            where
                O: DeserializeOwned + Send + Sync,
                C: helpers::dynamodb::HasDynamoDBConfiguration,
            {
                helpers::dynamodb::get_process_result_by_ticket(config, &self.ticket)
                    .await
            }
            /// Wait indefinitely for the ticket to be ready, and return when it is.
            pub async fn wait_until_complete(&self) -> Result<(), CoffeeShopError> {
                loop {
                    if let Some((_, status)) = self.result() {
                        {
                            crate::logger::init();
                            {
                                let lvl = ::log::Level::Info;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!(
                                            "Ticket {0} is ready, status: {1}.",
                                            self.ticket,
                                            status,
                                        ),
                                        lvl,
                                        &(
                                            LOG_TARGET,
                                            "coffeeshop::models::order",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                        };
                        return Ok(());
                    } else {
                        self.notify.notified().await;
                    }
                }
            }
            /// Wait for the ticket to be ready, and get the result when it is.
            ///
            /// The version of this function with a timeout is implemented as part of [`Shop`].
            pub async fn wait_and_fetch_when_complete<O, C>(
                &self,
                config: &C,
            ) -> Result<ProcessResultExport<O>, CoffeeShopError>
            where
                O: DeserializeOwned + Send + Sync,
                C: helpers::dynamodb::HasDynamoDBConfiguration,
            {
                self.wait_until_complete().await?;
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!(
                                    "Ticket {0} is ready, fetching the result...",
                                    self.ticket,
                                ),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::models::order",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                self.fetch(config).await
            }
        }
    }
    pub use order::{Order, Orders};
    mod shop {
        //! This module contains the models for the [`Shop`].
        mod base {
            use hashbrown::HashMap;
            use serde::{de::DeserializeOwned, Serialize};
            use std::{marker::PhantomData, sync::Arc};
            use tokio::sync::RwLock;
            use super::super::{
                message, Announcer, Barista, Machine, Order, Orders, Ticket, Waiter,
            };
            use crate::{cli::Config, helpers, CoffeeShopError};
            /// The default prefix for dynamodb table.
            const DYNAMODB_TABLE_PREFIX: &str = "task-queue-";
            /// The default prefix for SQS queue.
            const SQS_QUEUE_PREFIX: &str = "task-queue-";
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
            /// [`Shop`] can have a different number of baristas within it, but will always have one
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
            pub struct Shop<Q, I, O, F>
            where
                Q: message::QueryType,
                I: Serialize + DeserializeOwned + Send + Sync,
                O: Serialize + DeserializeOwned + Send + Sync,
                F: Machine<Q, I, O>,
            {
                /// The name of the task that this shop is responsible for.
                ///
                /// This is used to ensure multicast messages are only processed by the correct shop.
                /// Ideally, each shop should use unique multicast addresses to prevent message collisions.
                pub name: String,
                /// A map of tickets to their respective [`Notify`] events that are used to notify the
                /// waiter when a ticket is ready.
                pub orders: RwLock<Orders>,
                /// The coffee machine that will process tickets.
                ///
                /// This is the actual task that will be executed when a ticket is received. It should be able
                /// to tell apart any different types of tickets among the generic input type `I`, and produce
                /// a generic output type `O` regardless of the input type.
                pub coffee_machine: F,
                /// Dynamodb table name to store the finished products.
                pub dynamodb_table: String,
                /// The SQS queue name to store the tickets.
                pub sqs_queue: String,
                /// The configuration for the shop.
                ///
                /// These include the settings for the multicast address, the port, and the IP address, number
                /// of baristas etc.
                pub config: Config,
                /// The AWS SDK configuration for the shop.
                pub aws_config: helpers::aws::SdkConfig,
                /// Temporary Directory for serialization and deserialization.
                pub(crate) temp_dir: tempfile::TempDir,
                /// Reference to the waiter that will serve incoming requests.
                pub waiter: Arc<Waiter<Q, I, O, F>>,
                /// Reference to the baristas that will process the tickets.
                pub baristas: Vec<Barista<Q, I, O, F>>,
                /// Reference to the announcer that will announce the ticket is ready.
                pub announcer: Announcer<Q, I, O, F>,
                /// Phantom data to attach the input and output types to the shop.
                _phantom: PhantomData<(Q, I, O)>,
            }
            #[automatically_derived]
            impl<
                Q: ::core::fmt::Debug,
                I: ::core::fmt::Debug,
                O: ::core::fmt::Debug,
                F: ::core::fmt::Debug,
            > ::core::fmt::Debug for Shop<Q, I, O, F>
            where
                Q: message::QueryType,
                I: Serialize + DeserializeOwned + Send + Sync,
                O: Serialize + DeserializeOwned + Send + Sync,
                F: Machine<Q, I, O>,
            {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "name",
                        "orders",
                        "coffee_machine",
                        "dynamodb_table",
                        "sqs_queue",
                        "config",
                        "aws_config",
                        "temp_dir",
                        "waiter",
                        "baristas",
                        "announcer",
                        "_phantom",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.name,
                        &self.orders,
                        &self.coffee_machine,
                        &self.dynamodb_table,
                        &self.sqs_queue,
                        &self.config,
                        &self.aws_config,
                        &self.temp_dir,
                        &self.waiter,
                        &self.baristas,
                        &self.announcer,
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
            impl<Q, I, O, F> Shop<Q, I, O, F>
            where
                Q: message::QueryType,
                I: Serialize + DeserializeOwned + Send + Sync,
                O: Serialize + DeserializeOwned + Send + Sync,
                F: Machine<Q, I, O>,
            {
                /// Create a new shop with the given name, coffee machine, and configuration.
                pub async fn new(
                    name: String,
                    coffee_machine: F,
                    mut config: Config,
                    aws_config: Option<helpers::aws::SdkConfig>,
                    barista_count: usize,
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
                    let sqs_queue = config
                        .sqs_queue
                        .take()
                        .unwrap_or_else(|| ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}{1}", SQS_QUEUE_PREFIX, &name),
                            );
                            res
                        }));
                    let aws_config = if let Some(aws_config) = aws_config {
                        aws_config
                    } else {
                        helpers::aws::get_aws_config().await?
                    };
                    let temp_dir = tempfile::TempDir::new()
                        .map_err(|err| CoffeeShopError::TempDirCreationFailure(
                            err.to_string(),
                        ))?;
                    let shop = Arc::new_cyclic(|me| Self {
                        name,
                        orders: HashMap::new().into(),
                        coffee_machine,
                        dynamodb_table,
                        sqs_queue,
                        config,
                        aws_config,
                        temp_dir,
                        waiter: Arc::new(Waiter::new(me.clone())),
                        baristas: (0..barista_count)
                            .map(|_| Barista::new(me.clone()))
                            .collect::<Vec<Barista<Q, I, O, F>>>(),
                        announcer: Announcer::new(me.clone()),
                        _phantom: PhantomData,
                    });
                    '_init: {
                        shop.announcer.init()?;
                    }
                    Ok(shop)
                }
                /// Check if this shop has an order for a given ticket.
                pub async fn has_order(&self, ticket: &Ticket) -> bool {
                    self.orders.read().await.contains_key(ticket)
                }
                /// Get the order for a given ticket in the shop.
                pub async fn get_order(&self, ticket: &Ticket) -> Option<Arc<Order>> {
                    self.orders.read().await.get(ticket).cloned()
                }
                /// Spawn a [`Order`] order for a given [`Ticket`] in the shop.
                ///
                /// Get the ticket if it exists, otherwise create a new one
                /// before returning the [`Arc`] reference to the [`Order`].
                pub async fn spawn_order(&self, ticket: Ticket) -> Arc<Order> {
                    self.orders
                        .write()
                        .await
                        .entry(ticket.clone())
                        .or_insert_with_key(|_| Arc::new(Order::new(ticket)))
                        .clone()
                }
            }
        }
        pub use base::*;
        mod open {
            use super::*;
            use std::sync::Arc;
            use tokio::sync::Notify;
            use serde::{de::DeserializeOwned, Serialize};
            use crate::{
                helpers, models::{message, Machine, Barista},
                CoffeeShopError,
            };
            impl<Q, I, O, F> Shop<Q, I, O, F>
            where
                Q: message::QueryType + 'static,
                I: Serialize + DeserializeOwned + Send + Sync + 'static,
                O: Serialize + DeserializeOwned + Send + Sync + 'static,
                F: Machine<Q, I, O> + 'static,
            {
                /// Open the shop, start listening for requests.
                pub async fn open(&self) -> Result<(), CoffeeShopError> {
                    helpers::sts::report_aws_login(Some(&self.aws_config)).await?;
                    let shutdown_signal = Arc::new(Notify::new());
                    let max_execution_time = self.config.max_execution_time();
                    let result: Result<((), (), (), ()), CoffeeShopError> = {
                        use ::tokio::macros::support::{maybe_done, poll_fn, Future, Pin};
                        use ::tokio::macros::support::Poll::{Ready, Pending};
                        let mut futures = (
                            maybe_done(async {
                                tokio::signal::ctrl_c().await;
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Warn;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Received termination signal. Shutting down the shop.",
                                                ),
                                                lvl,
                                                &(
                                                    "coffeeshop::models::shop::open",
                                                    "coffeeshop::models::shop::open",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                };
                                shutdown_signal.clone().notify_waiters();
                                Ok::<_, CoffeeShopError>(())
                            }),
                            maybe_done(async {
                                self.waiter
                                    .serve(
                                        ::alloc::vec::Vec::new().into_iter(),
                                        shutdown_signal.clone(),
                                        max_execution_time,
                                    )
                                    .inspect_err(|err| {
                                        crate::logger::init();
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "The waiter has stopped serving requests. Error: {0:?}",
                                                        err,
                                                    ),
                                                    lvl,
                                                    &(
                                                        "coffeeshop::models::shop::open",
                                                        "coffeeshop::models::shop::open",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                    })
                            }),
                            maybe_done(async {
                                Barista::serve_all(&self.baristas, shutdown_signal.clone())
                                    .inspect_err(|err| {
                                        crate::logger::init();
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "The baristas have stopped serving requests. Error: {0:?}",
                                                        err,
                                                    ),
                                                    lvl,
                                                    &(
                                                        "coffeeshop::models::shop::open",
                                                        "coffeeshop::models::shop::open",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                    })
                            }),
                            maybe_done(async {
                                self.announcer
                                    .listen_for_announcements(shutdown_signal.clone())
                                    .inspect_err(|err| {
                                        crate::logger::init();
                                        {
                                            let lvl = ::log::Level::Error;
                                            if lvl <= ::log::STATIC_MAX_LEVEL
                                                && lvl <= ::log::max_level()
                                            {
                                                ::log::__private_api::log(
                                                    format_args!(
                                                        "The announcer has stopped listening for announcements. Error: {0:?}",
                                                        err,
                                                    ),
                                                    lvl,
                                                    &(
                                                        "coffeeshop::models::shop::open",
                                                        "coffeeshop::models::shop::open",
                                                        ::log::__private_api::loc(),
                                                    ),
                                                    (),
                                                );
                                            }
                                        };
                                    })
                            }),
                        );
                        let mut futures = &mut futures;
                        let mut skip_next_time: u32 = 0;
                        poll_fn(move |cx| {
                                const COUNT: u32 = 0 + 1 + 1 + 1 + 1;
                                let mut is_pending = false;
                                let mut to_run = COUNT;
                                let mut skip = skip_next_time;
                                skip_next_time = if skip + 1 == COUNT {
                                    0
                                } else {
                                    skip + 1
                                };
                                loop {
                                    if skip == 0 {
                                        if to_run == 0 {
                                            break;
                                        }
                                        to_run -= 1;
                                        let (fut, ..) = &mut *futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        if fut.as_mut().poll(cx).is_pending() {
                                            is_pending = true;
                                        } else if fut
                                            .as_mut()
                                            .output_mut()
                                            .expect("expected completed future")
                                            .is_err()
                                        {
                                            return Ready(
                                                Err(
                                                    fut
                                                        .take_output()
                                                        .expect("expected completed future")
                                                        .err()
                                                        .unwrap(),
                                                ),
                                            )
                                        }
                                    } else {
                                        skip -= 1;
                                    }
                                    if skip == 0 {
                                        if to_run == 0 {
                                            break;
                                        }
                                        to_run -= 1;
                                        let (_, fut, ..) = &mut *futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        if fut.as_mut().poll(cx).is_pending() {
                                            is_pending = true;
                                        } else if fut
                                            .as_mut()
                                            .output_mut()
                                            .expect("expected completed future")
                                            .is_err()
                                        {
                                            return Ready(
                                                Err(
                                                    fut
                                                        .take_output()
                                                        .expect("expected completed future")
                                                        .err()
                                                        .unwrap(),
                                                ),
                                            )
                                        }
                                    } else {
                                        skip -= 1;
                                    }
                                    if skip == 0 {
                                        if to_run == 0 {
                                            break;
                                        }
                                        to_run -= 1;
                                        let (_, _, fut, ..) = &mut *futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        if fut.as_mut().poll(cx).is_pending() {
                                            is_pending = true;
                                        } else if fut
                                            .as_mut()
                                            .output_mut()
                                            .expect("expected completed future")
                                            .is_err()
                                        {
                                            return Ready(
                                                Err(
                                                    fut
                                                        .take_output()
                                                        .expect("expected completed future")
                                                        .err()
                                                        .unwrap(),
                                                ),
                                            )
                                        }
                                    } else {
                                        skip -= 1;
                                    }
                                    if skip == 0 {
                                        if to_run == 0 {
                                            break;
                                        }
                                        to_run -= 1;
                                        let (_, _, _, fut, ..) = &mut *futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        if fut.as_mut().poll(cx).is_pending() {
                                            is_pending = true;
                                        } else if fut
                                            .as_mut()
                                            .output_mut()
                                            .expect("expected completed future")
                                            .is_err()
                                        {
                                            return Ready(
                                                Err(
                                                    fut
                                                        .take_output()
                                                        .expect("expected completed future")
                                                        .err()
                                                        .unwrap(),
                                                ),
                                            )
                                        }
                                    } else {
                                        skip -= 1;
                                    }
                                }
                                if is_pending {
                                    Pending
                                } else {
                                    Ready(
                                        Ok((
                                            {
                                                let (fut, ..) = &mut futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                fut.take_output()
                                                    .expect("expected completed future")
                                                    .ok()
                                                    .expect("expected Ok(_)")
                                            },
                                            {
                                                let (_, fut, ..) = &mut futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                fut.take_output()
                                                    .expect("expected completed future")
                                                    .ok()
                                                    .expect("expected Ok(_)")
                                            },
                                            {
                                                let (_, _, fut, ..) = &mut futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                fut.take_output()
                                                    .expect("expected completed future")
                                                    .ok()
                                                    .expect("expected Ok(_)")
                                            },
                                            {
                                                let (_, _, _, fut, ..) = &mut futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                fut.take_output()
                                                    .expect("expected completed future")
                                                    .ok()
                                                    .expect("expected Ok(_)")
                                            },
                                        )),
                                    )
                                }
                            })
                            .await
                    };
                    result.map(|_| ())
                }
            }
        }
        mod implementations {
            //! This module contains the implementations of the shop models.
            //!
            //! Since implementations do not need to be referenced, these modules do not need to be
            //! public.
            use super::Shop;
            mod has_aws_sdk_config {
                use crate::{helpers::aws, models::{message, Machine}};
                use serde::{de::DeserializeOwned, Serialize};
                use super::Shop;
                impl<Q, I, O, F> aws::HasAWSSdkConfig for Shop<Q, I, O, F>
                where
                    Q: message::QueryType,
                    I: Serialize + DeserializeOwned + Send + Sync,
                    O: Serialize + DeserializeOwned + Send + Sync,
                    F: Machine<Q, I, O>,
                {
                    /// The AWS SDK configuration for the shop.
                    fn aws_config(&self) -> &aws::SdkConfig {
                        &self.aws_config
                    }
                }
            }
            mod has_dynamodb_config {
                use crate::{helpers::dynamodb, models::{message, Machine}};
                use serde::{de::DeserializeOwned, Serialize};
                use super::Shop;
                impl<Q, I, O, F> dynamodb::HasDynamoDBConfiguration for Shop<Q, I, O, F>
                where
                    Q: message::QueryType,
                    I: Serialize + DeserializeOwned + Send + Sync,
                    O: Serialize + DeserializeOwned + Send + Sync,
                    F: Machine<Q, I, O>,
                {
                    fn dynamodb_table(&self) -> &str {
                        &self.dynamodb_table
                    }
                    fn dynamodb_partition_key(&self) -> &str {
                        &self.config.dynamodb_partition_key
                    }
                    /// The time-to-live (TTL) duration for the items in the DynamoDB table.
                    fn dynamodb_ttl(&self) -> tokio::time::Duration {
                        self.config.dynamodb_ttl()
                    }
                }
            }
            mod has_sqs_config {
                use crate::{helpers::sqs, models::{message, Machine}};
                use serde::{de::DeserializeOwned, Serialize};
                use super::Shop;
                impl<Q, I, O, F> sqs::HasSQSConfiguration for Shop<Q, I, O, F>
                where
                    Q: message::QueryType,
                    I: Serialize + DeserializeOwned + Send + Sync,
                    O: Serialize + DeserializeOwned + Send + Sync,
                    F: Machine<Q, I, O>,
                {
                    /// The SQS queue URL for the shop.
                    fn sqs_queue_url(&self) -> &str {
                        &self.sqs_queue
                    }
                }
            }
            mod collection_point {
                use crate::{
                    helpers::dynamodb::{self, HasDynamoDBConfiguration},
                    models::{message, Machine, Orders, Shop},
                    CoffeeShopError,
                };
                use serde::{de::DeserializeOwned, Serialize};
                use tokio::sync::RwLock;
                const LOG_TARGET: &str = "coffeeshop::models::collection_point";
                /// A [`CollectionPoint`] is a behaviour of a [`Shop`] that:
                /// - Monitors the orders on DynamoDB that is flagged by the [`Waiter`]s
                /// - Listens for Multicast messages from the [`Barista`]s from this and other [`Shop`]s
                /// - Update the [`Order`]s in the [`Shop`] instance with the results from the [`Barista`]s
                pub trait CollectionPoint: HasDynamoDBConfiguration {
                    /// Access the orders relevant to the collection point.
                    fn orders(&self) -> &RwLock<Orders>;
                    /// Purge stale orders from the collection point.
                    ///
                    /// Currently, this function never fails; the error type is reserved for future use.
                    #[must_use]
                    #[allow(
                        elided_named_lifetimes,
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::needless_arbitrary_self_type,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn purge_stale_orders<'life0, 'async_trait>(
                        &'life0 self,
                        max_age: tokio::time::Duration,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), CoffeeShopError>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: ::core::marker::Sync + 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<(), CoffeeShopError>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let __self = self;
                            let max_age = max_age;
                            let __ret: Result<(), CoffeeShopError> = {
                                let mut orders = __self.orders().write().await;
                                let removed = orders
                                    .extract_if(|_k, v| v.is_stale(max_age));
                                {
                                    crate::logger::init();
                                    {
                                        let lvl = ::log::Level::Info;
                                        if lvl <= ::log::STATIC_MAX_LEVEL
                                            && lvl <= ::log::max_level()
                                        {
                                            ::log::__private_api::log(
                                                format_args!(
                                                    "Purged {0} stale orders from the collection point.",
                                                    removed.count(),
                                                ),
                                                lvl,
                                                &(
                                                    LOG_TARGET,
                                                    "coffeeshop::models::shop::implementations::collection_point",
                                                    ::log::__private_api::loc(),
                                                ),
                                                (),
                                            );
                                        }
                                    };
                                };
                                Ok(())
                            };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    /// Periodically purge stale orders from the collection point.
                    ///
                    /// This function will loop indefinitely until the program is terminated.
                    #[must_use]
                    #[allow(
                        elided_named_lifetimes,
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::needless_arbitrary_self_type,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn periodic_purge_stale_orders<'life0, 'async_trait>(
                        &'life0 self,
                        max_age: tokio::time::Duration,
                        interval: tokio::time::Duration,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = (),
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: ::core::marker::Sync + 'async_trait,
                    {
                        Box::pin(async move {
                            let __self = self;
                            let max_age = max_age;
                            let interval = interval;
                            let () = {
                                loop {
                                    tokio::time::sleep(interval).await;
                                    if let Err(err) = __self.purge_stale_orders(max_age).await {
                                        {
                                            crate::logger::init();
                                            {
                                                let lvl = ::log::Level::Error;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api::log(
                                                        format_args!(
                                                            "Failed to purge stale orders from the collection point: {0}",
                                                            err,
                                                        ),
                                                        lvl,
                                                        &(
                                                            LOG_TARGET,
                                                            "coffeeshop::models::shop::implementations::collection_point",
                                                            ::log::__private_api::loc(),
                                                        ),
                                                        (),
                                                    );
                                                }
                                            };
                                        };
                                    }
                                }
                            };
                        })
                    }
                }
                impl<Q, I, O, F> CollectionPoint for Shop<Q, I, O, F>
                where
                    Q: message::QueryType,
                    I: Serialize + DeserializeOwned + Send + Sync,
                    O: Serialize + DeserializeOwned + Send + Sync,
                    F: Machine<Q, I, O>,
                {
                    /// Access the orders in the [`Shop`] instance.
                    fn orders(&self) -> &RwLock<Orders> {
                        &self.orders
                    }
                }
                /// These methods could not form part of the [`CollectionPoint`] trait because they
                /// uses the `self` reference in a way that requires too many lifetimes to be specified.
                impl<Q, I, O, F> Shop<Q, I, O, F>
                where
                    Q: message::QueryType,
                    I: Serialize + DeserializeOwned + Send + Sync,
                    O: Serialize + DeserializeOwned + Send + Sync,
                    F: Machine<Q, I, O>,
                {
                    /// Listen to the multicast messages from the [`Barista`]s.
                    ///
                    /// This function never returns; it will simply listen for multicast messages
                    /// and spawn handlers for each received message to update the [`Order`]s.
                    ///
                    /// # Note
                    ///
                    /// Internal function: this function is not meant to be called directly.
                    pub async fn listen_for_multicast(
                        &self,
                    ) -> Result<(), CoffeeShopError> {
                        ::core::panicking::panic("not yet implemented")
                    }
                    /// Check DynamoDB for newly fulfilled [`Order`]s.
                    ///
                    /// # Note
                    ///
                    /// Internal function: this function is not meant to be called directly.
                    pub async fn check_for_fulfilled_orders(
                        &self,
                    ) -> Result<(), CoffeeShopError> {
                        let found_results = async {
                            let orders = self.orders().read().await;
                            let unfulfilled_tickets = orders
                                .iter()
                                .filter_map(|(k, v)| (!v.is_fulfilled()).then_some(k))
                                .cloned()
                                .collect::<Vec<_>>();
                            drop(orders);
                            dynamodb::get_process_successes_by_tickets::<
                                _,
                            >(self, unfulfilled_tickets.iter())
                                .await
                        }
                            .await?;
                        let orders = self.orders().read().await;
                        for (ticket, result) in found_results {
                            if let Some(order) = orders.get(&ticket) {
                                order.complete(result)?;
                            }
                        }
                        Ok(())
                    }
                }
            }
            pub use collection_point::CollectionPoint;
        }
        pub use implementations::*;
    }
    pub use shop::*;
    mod machine {
        use crate::{CoffeeMachineError, ValidationError};
        use axum::http;
        use serde::{de::DeserializeOwned, Serialize};
        use super::message;
        /// A trait that defines the behavior of a coffee machine, i.e. the function
        /// that will be called when a ticket is received, and outputs the result
        /// to the DynamoDB table.
        ///
        /// # Lifetime
        ///
        /// A [`Machine`] is not required to have a `'static` lifetime, it just needs
        /// to be owned by the [`Shop`] and be available for that duration. It is
        /// possible - while not RESTful - to have a stateful machine that gives out
        /// different responses based on its state using [`RwLock`](tokio::sync::RwLock)
        /// or [`Mutex`](tokio::sync::Mutex) or the like.
        ///
        /// However it is worth noting that the [`Machine`]s will not synchronise with
        /// each other across [`Shop`], unless you implement a mechanism to do so. Thus
        /// the internal state of the [`Machine`] should be considered ephemeral and
        /// typically limited to caching or other non-critical data. Use a database
        /// if you need to share state across multiple [`Machine`]s.
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
        pub trait Machine<Q, I, O>: Send + Sync + Sized
        where
            Q: message::QueryType,
            I: DeserializeOwned + Serialize + Send + Sync,
            O: DeserializeOwned + Serialize + Send + Sync,
        {
            /// Required method for the [`Machine`] trait.
            ///
            /// A [`Machine`] is expected to process the input and return the output; if an error
            /// occurs, it should return a [`CoffeeMachineError`].
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn call<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 self,
                query: &'life1 Q,
                input: Option<&'life2 I>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = message::MachineResult<O>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait;
            /// Validate the input before processing.
            ///
            /// This prevents erroronous input from being sent to the SQS in the first place;
            /// and the [`Waiter`] will return a [`http::StatusCode::UNPROCESSABLE_ENTITY`] response
            /// with the given [`ValidationError`] as [details](serde_json::Value).
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds
            )]
            fn validator<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 self,
                query: &'life1 Q,
                input: Option<&'life2 I>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<(), ValidationError>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait;
            /// The default validator implementation that wraps the [details](serde_json::Value)
            /// in a [`CoffeeMachineError`] and returns it.
            ///
            /// Due to the signature of this method, you can optionally call it at the top of your
            /// [`call`](Self::call) method to ensure that the input is valid before processing.
            /// This is useful if you have other queue inputs that are not validated by
            /// the [`Waiter`].
            ///
            /// # Note
            ///
            /// You can customise this method to return a different error as needed, as long
            /// as it returns a [`CoffeeMachineError`].
            #[must_use]
            #[allow(
                elided_named_lifetimes,
                clippy::async_yields_async,
                clippy::diverging_sub_expression,
                clippy::let_unit_value,
                clippy::needless_arbitrary_self_type,
                clippy::no_effect_underscore_binding,
                clippy::shadow_same,
                clippy::type_complexity,
                clippy::type_repetition_in_bounds,
                clippy::used_underscore_binding
            )]
            fn validate<'life0, 'life1, 'life2, 'async_trait>(
                &'life0 self,
                query: &'life1 Q,
                input: Option<&'life2 I>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                        Output = Result<(), CoffeeMachineError>,
                    > + ::core::marker::Send + 'async_trait,
                >,
            >
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                'life2: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                        Result<(), CoffeeMachineError>,
                    > {
                        #[allow(unreachable_code)] return __ret;
                    }
                    let __self = self;
                    let input = input;
                    let __ret: Result<(), CoffeeMachineError> = {
                        __self
                            .validator(query, input)
                            .await
                            .map_err(|details| CoffeeMachineError::new(
                                http::StatusCode::UNPROCESSABLE_ENTITY,
                                "ValidationError".to_owned(),
                                Some(
                                    ::serde_json::Value::Object({
                                        let mut object = ::serde_json::Map::new();
                                        let _ = object
                                            .insert(
                                                ("message").into(),
                                                ::serde_json::to_value(
                                                        &::alloc::__export::must_use({
                                                            let res = ::alloc::fmt::format(
                                                                format_args!(
                                                                    "Input cannot be validated. Please see details for {0} invalid field{1} below.",
                                                                    details.len(),
                                                                    if details.len() == 1 { "" } else { "s" },
                                                                ),
                                                            );
                                                            res
                                                        }),
                                                    )
                                                    .unwrap(),
                                            );
                                        let _ = object
                                            .insert(
                                                ("fields").into(),
                                                ::serde_json::to_value(
                                                        &serde_json::to_value(details)
                                                            .expect(
                                                                "Failed to serialize the validation error details; this should not be possible.",
                                                            ),
                                                    )
                                                    .unwrap(),
                                            );
                                        object
                                    }),
                                ),
                            ))
                    };
                    #[allow(unreachable_code)] __ret
                })
            }
        }
    }
    pub use machine::Machine;
    mod waiter {
        //! A waiter is an async HTTP host that listens for incoming requests and insert them into
        //! the specified AWS SQS queue.
        //! For synchronous requests, the waiter will also asynchronously await a [`Notify`](tokio::sync::Notify)
        //! event from the multicast channel and report back to the client when the request had been processed.
        use std::{
            ops::Deref,
            sync::{
                atomic::{AtomicUsize, Ordering},
                Arc, Weak,
            },
        };
        use axum::extract::{Json, Query};
        use axum::{
            http::{header, StatusCode},
            response::IntoResponse,
        };
        use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
        use tokio::sync::Notify;
        use super::{
            message::{self, QueryType},
            Machine, Order, Shop,
        };
        use crate::{helpers, CoffeeShopError};
        const LOG_TARGET: &str = "coffeeshop::models::waiter";
        /// A [`Waiter`] instance that acts as an async REST API host.
        pub struct Waiter<Q, I, O, F>
        where
            Q: message::QueryType,
            I: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
            O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            /// The back reference to the shop that this waiter is serving.
            pub shop: Weak<Shop<Q, I, O, F>>,
            /// The total amount of historical requests processed.
            /// Only the [`request`](Self::request) and [`async_request`](Self::async_request) methods
            /// will increment this counter.
            ///
            /// Internally, this is done by [`create_ticket`](Self::create_ticket).
            pub request_count: Arc<AtomicUsize>,
            pub start_time: tokio::time::Instant,
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
            I: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
            O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Waiter",
                    "shop",
                    &self.shop,
                    "request_count",
                    &self.request_count,
                    "start_time",
                    &&self.start_time,
                )
            }
        }
        impl<Q, I, O, F> Waiter<Q, I, O, F>
        where
            Q: message::QueryType,
            I: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
            O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync,
            F: Machine<Q, I, O>,
        {
            /// Create a new [`Waiter`] instance.
            pub fn new(shop: Weak<Shop<Q, I, O, F>>) -> Self {
                Self {
                    shop,
                    request_count: Arc::new(AtomicUsize::new(0)),
                    start_time: tokio::time::Instant::now(),
                }
            }
            /// Get a reference to the shop that this waiter is serving.
            pub fn shop(&self) -> Arc<Shop<Q, I, O, F>> {
                self.shop
                    .upgrade()
                    .expect(
                        "Shop has been dropped; this should not be possible in normal use. Please report this to the maintainer.",
                    )
            }
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
                        ticket_count: self.shop().orders.read().await.len(),
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
                self.create_and_retrieve_order(
                        message::CombinedInput::new(params, Some(payload)),
                        timeout,
                    )
                    .await
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
                self.create_order(message::CombinedInput {
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
                self.retrieve_order_with_timeout(params.ticket, timeout).await
            }
            /// An internal method to create a new ticket on the AWS SQS queue,
            /// then return the [`Order`] instance to await the result.
            pub async fn create_order(
                &self,
                input: message::CombinedInput<Q, I>,
            ) -> Result<(message::Ticket, Arc<Order>), CoffeeShopError> {
                let shop = self.shop();
                shop.coffee_machine
                    .validate(&input.query, input.input.as_ref())
                    .await
                    .inspect_err(|err| {
                        crate::logger::init();
                        {
                            let lvl = ::log::Level::Warn;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!(
                                        "Validation failed, not pushing to SQS: {0:#?}",
                                        err,
                                    ),
                                    lvl,
                                    &(
                                        LOG_TARGET,
                                        "coffeeshop::models::waiter",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        };
                    })
                    .map_err(CoffeeShopError::ErrorSchema)?;
                self.request_count.fetch_add(1, Ordering::Relaxed);
                let ticket = helpers::sqs::put_ticket(
                        shop.deref(),
                        input,
                        &shop.temp_dir,
                    )
                    .await?;
                Ok((ticket.clone(), shop.spawn_order(ticket).await))
            }
            /// An internal method to retrieve the result of a ticket from the
            /// AWS SQS queue.
            pub async fn retrieve_order<'o>(
                &self,
                ticket: String,
            ) -> axum::response::Response {
                let start_time = self.start_time;
                let shop = self.shop();
                let order = shop
                    .orders
                    .read()
                    .await
                    .get(&ticket)
                    .ok_or_else(|| CoffeeShopError::TicketNotFound(ticket.clone()))
                    .map(Arc::clone);
                match &order {
                    tmp => {
                        {
                            ::std::io::_eprint(
                                format_args!(
                                    "[{0}:{1}:{2}] {3} = {4:#?}\n",
                                    "src/models/waiter.rs",
                                    172u32,
                                    9u32,
                                    "&order",
                                    &tmp,
                                ),
                            );
                        };
                        tmp
                    }
                };
                if let Err(err) = order {
                    return err.into_response();
                }
                let order = order.unwrap();
                {
                    crate::logger::init();
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!(
                                    "Waiting for order {0} to complete...",
                                    ticket,
                                ),
                                lvl,
                                &(
                                    LOG_TARGET,
                                    "coffeeshop::models::waiter",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                };
                order
                    .wait_and_fetch_when_complete::<O, _>(shop.deref())
                    .await
                    .map(|result| {
                        result
                            .map(|output| {
                                message::OutputResponse::new(ticket, &output, &start_time)
                                    .into_response()
                            })
                    })
                    .into_response()
            }
            /// An internal method to retrieve the result of a ticket with a timeout;
            /// if the timeout is reached, an [`CoffeeShopError::RetrieveTimeout`] is returned.
            pub async fn retrieve_order_with_timeout(
                &self,
                ticket: String,
                timeout: Option<tokio::time::Duration>,
            ) -> axum::response::Response {
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
                                self.retrieve_order(ticket),
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
                                Err::<(), _>(CoffeeShopError::RetrieveTimeout(timeout))
                                    .into_response()
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
                    self.retrieve_order(ticket).await
                }
            }
            /// An internal method to create a new ticket, wait for the result,
            /// then return the result to the client.
            ///
            /// The `timeout` parameter is used to set a timeout for the processing and
            /// retrieval of the ticket only; the creation of the ticket is not affected.
            pub async fn create_and_retrieve_order(
                &self,
                input: message::CombinedInput<Q, I>,
                timeout: Option<tokio::time::Duration>,
            ) -> axum::response::Response {
                match self.create_order(input).await {
                    Ok((ticket, _order)) => {
                        self.retrieve_order_with_timeout(ticket, timeout).await
                    }
                    Err(err) => err.into_response(),
                }
            }
        }
        /// Implementation for the waiter where the query types are 'static.
        impl<Q, I, O, F> Waiter<Q, I, O, F>
        where
            Q: message::QueryType + 'static,
            I: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
            O: serde::Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
            F: Machine<Q, I, O> + 'static,
        {
            /// Start an [`axum`] app and serve incoming requests.
            pub async fn serve(
                self: &Arc<Self>,
                additional_routes: impl Iterator<
                    Item = (
                        &'static str,
                        axum::routing::method_routing::MethodRouter<()>,
                    ),
                >,
                shutdown_signal: Arc<Notify>,
                max_execution_time: Option<tokio::time::Duration>,
            ) -> Result<(), CoffeeShopError> {
                let mut app = axum::Router::new()
                    .route(
                        "/status",
                        axum::routing::get({
                            let arc_self = Arc::clone(self);
                            || async move { arc_self.status().await }
                        }),
                    )
                    .route(
                        "/request",
                        axum::routing::post({
                            let arc_self = Arc::clone(self);
                            |Query(params): Query<Q>, body: Json<I>| async move {
                                if params.is_async() {
                                    arc_self
                                        .async_request(Query(params), body)
                                        .await
                                        .into_response()
                                } else {
                                    arc_self.request(Query(params), body).await.into_response()
                                }
                            }
                        }),
                    )
                    .route(
                        "/retrieve",
                        axum::routing::get({
                            let arc_self = Arc::clone(self);
                            |query| async move { arc_self.async_retrieve(query).await }
                        }),
                    );
                app = additional_routes
                    .fold(app, |app, (path, handler)| app.route(path, handler));
                if let Some(max_execution_time) = max_execution_time {
                    app = app
                        .layer((
                            TraceLayer::new_for_http(),
                            TimeoutLayer::new(max_execution_time),
                        ));
                }
                let listener = tokio::net::TcpListener::bind(
                        self.shop().config.host_addr(),
                    )
                    .await
                    .map_err(|err| CoffeeShopError::ListenerCreationFailure(
                        err.to_string(),
                        self.shop().config.host_addr(),
                    ))?;
                let server = axum::serve(listener, app)
                    .with_graceful_shutdown(async move {
                        shutdown_signal.notified().await
                    });
                server.await.map_err(CoffeeShopError::from_server_io_error)
            }
        }
    }
    pub use waiter::*;
    pub mod message {
        //! This module contains the internal data structures for messaging between
        //! structs.
        mod input {
            use super::QueryType;
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
            impl<Q, I> CombinedInput<Q, I>
            where
                Q: QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                /// Create a new [`CombinedInput`] instance.
                pub fn new(query: Q, input: Option<I>) -> Self {
                    Self { query, input }
                }
            }
            /// Generated by `cargo expand` from `derive(Deserialize)` on `CombinedInput`.
            ///
            /// Manipulated to allow `'de` lifetime hidden from the public API. The normal
            /// `#[derive(Deserialize)]` would not work as it expects all fields to be
            /// non-Generic [`serde::Deserialize`] types, which in our case it is, but it
            /// could not infer the trait bounds correctly.
            ///
            /// Since we also require both `Q` and `I` to be [`serde::de::DeserializeOwned`]
            /// types, we can confine the `'de` lifetime to the hidden implementation.
            ///
            /// # Warning
            ///
            /// This might break if the `serde` crate changes its implementation, or if the
            /// `serde` crate changes its API. This is a fragile implementation, and should
            /// be used with caution.
            ///
            /// This implementation is generated from 1.0.215 of the `serde` crate.
            impl<'de, Q, I> serde::Deserialize<'de> for CombinedInput<Q, I>
            where
                Q: QueryType,
                I: serde::de::DeserializeOwned + serde::Serialize,
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum CombinedInputField {
                        Query,
                        Input,
                        Ignored,
                    }
                    #[doc(hidden)]
                    struct FieldVisitor;
                    impl serde::de::Visitor<'_> for FieldVisitor {
                        type Value = CombinedInputField;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter,
                        ) -> std::fmt::Result {
                            std::fmt::Formatter::write_str(formatter, "field identifier")
                        }
                        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            match value {
                                0u64 => Ok(CombinedInputField::Query),
                                1u64 => Ok(CombinedInputField::Input),
                                _ => Ok(CombinedInputField::Ignored),
                            }
                        }
                        fn visit_str<E>(self, __value: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            match __value {
                                "query" => Ok(CombinedInputField::Query),
                                "input" => Ok(CombinedInputField::Input),
                                _ => Ok(CombinedInputField::Ignored),
                            }
                        }
                        fn visit_bytes<E>(self, __value: &[u8]) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            match __value {
                                b"query" => Ok(CombinedInputField::Query),
                                b"input" => Ok(CombinedInputField::Input),
                                _ => Ok(CombinedInputField::Ignored),
                            }
                        }
                    }
                    impl<'de> serde::Deserialize<'de> for CombinedInputField {
                        #[inline]
                        fn deserialize<D>(__deserializer: D) -> Result<Self, D::Error>
                        where
                            D: serde::Deserializer<'de>,
                        {
                            serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de, Q, I>
                    where
                        Q: QueryType,
                        I: serde::de::DeserializeOwned + serde::Serialize,
                    {
                        marker: std::marker::PhantomData<CombinedInput<Q, I>>,
                        lifetime: std::marker::PhantomData<&'de ()>,
                    }
                    impl<'de, Q, I> serde::de::Visitor<'de> for __Visitor<'de, Q, I>
                    where
                        Q: QueryType,
                        I: serde::de::DeserializeOwned + serde::Serialize,
                    {
                        type Value = CombinedInput<Q, I>;
                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter,
                        ) -> std::fmt::Result {
                            std::fmt::Formatter::write_str(
                                formatter,
                                "struct CombinedInput",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut seq: __A,
                        ) -> Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::SeqAccess<'de>,
                        {
                            let query = match serde::de::SeqAccess::next_element::<
                                Q,
                            >(&mut seq)? {
                                Some(value) => value,
                                None => {
                                    return Err(
                                        serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct CombinedInput with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let input = match serde::de::SeqAccess::next_element::<
                                Option<I>,
                            >(&mut seq)? {
                                Some(value) => value,
                                None => {
                                    return Err(
                                        serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct CombinedInput with 2 elements",
                                        ),
                                    );
                                }
                            };
                            Ok(CombinedInput { query, input })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut map: __A,
                        ) -> Result<Self::Value, __A::Error>
                        where
                            __A: serde::de::MapAccess<'de>,
                        {
                            let mut query: Option<Q> = None;
                            let mut input: Option<Option<I>> = None;
                            while let Some(key) = serde::de::MapAccess::next_key::<
                                CombinedInputField,
                            >(&mut map)? {
                                match key {
                                    CombinedInputField::Query => {
                                        if Option::is_some(&query) {
                                            return Err(
                                                <__A::Error as serde::de::Error>::duplicate_field("query"),
                                            );
                                        }
                                        query = Some(
                                            serde::de::MapAccess::next_value::<Q>(&mut map)?,
                                        );
                                    }
                                    CombinedInputField::Input => {
                                        if Option::is_some(&input) {
                                            return Err(
                                                <__A::Error as serde::de::Error>::duplicate_field("input"),
                                            );
                                        }
                                        input = Some(
                                            serde::de::MapAccess::next_value::<Option<I>>(&mut map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = serde::de::MapAccess::next_value::<
                                            serde::de::IgnoredAny,
                                        >(&mut map)?;
                                    }
                                }
                            }
                            let query = match query {
                                Some(value) => value,
                                None => serde::__private::de::missing_field("query")?,
                            };
                            let input = match input {
                                Some(value) => value,
                                None => serde::__private::de::missing_field("input")?,
                            };
                            Ok(CombinedInput { query, input })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &[&str] = &["query", "input"];
                    serde::Deserializer::deserialize_struct(
                        deserializer,
                        "CombinedInput",
                        FIELDS,
                        __Visitor {
                            marker: std::marker::PhantomData::<CombinedInput<Q, I>>,
                            lifetime: std::marker::PhantomData,
                        },
                    )
                }
            }
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
        mod process_result {
            use crate::errors::{CoffeeMachineError, CoffeeShopError, ErrorSchema};
            /// A type alias for the result of processing a ticket before serializing it into DynamoDB.
            pub type ProcessResult<O> = Result<O, CoffeeShopError>;
            /// A type alias for the result of processing a ticket after retrieving it from DynamoDB.
            ///
            /// The original error type will not be preserved as the origina error could contain
            /// non-serializable types or non-static lifetimes.
            pub type ProcessResultExport<O> = Result<O, ErrorSchema>;
            /// A type alias for the result of calling the coffee machine.
            pub type MachineResult<O> = Result<O, CoffeeMachineError>;
        }
        pub use process_result::*;
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
                pub ticket: ::prost::alloc::string::String,
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
                        ticket: ::core::clone::Clone::clone(&self.ticket),
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
                    self.task == other.task && self.ticket == other.ticket
                        && self.kind == other.kind && self.timestamp == other.timestamp
                        && self.status == other.status
                }
            }
            impl ::prost::Message for MulticastMessage {
                #[allow(unused_variables)]
                fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                    if self.ticket != "" {
                        ::prost::encoding::string::encode(1u32, &self.ticket, buf);
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
                            let mut value = &mut self.ticket;
                            ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                .map_err(|mut error| {
                                    error.push(STRUCT_NAME, "ticket");
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
                        + if self.ticket != "" {
                            ::prost::encoding::string::encoded_len(1u32, &self.ticket)
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
                    self.ticket.clear();
                    self.kind = multicast_message::Kind::default() as i32;
                    self.timestamp = ::core::option::Option::None;
                    self.status = multicast_message::Status::default() as i32;
                    self.task.clear();
                }
            }
            impl ::core::default::Default for MulticastMessage {
                fn default() -> Self {
                    MulticastMessage {
                        ticket: ::prost::alloc::string::String::new(),
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
                            ScalarWrapper(&self.ticket)
                        };
                        builder.field("ticket", &wrapper)
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
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
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
                                ::core::result::Result::Err(
                                    ::prost::UnknownEnumValue(value),
                                )
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
                    Failure = 2,
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
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(
                            f,
                            match self {
                                Status::Rejected => "Rejected",
                                Status::Complete => "Complete",
                                Status::Failure => "Failure",
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
                        ::core::cmp::PartialOrd::partial_cmp(
                            &__self_discr,
                            &__arg1_discr,
                        )
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
                            2 => true,
                            _ => false,
                        }
                    }
                    #[deprecated = "Use the TryFrom<i32> implementation instead"]
                    ///Converts an `i32` to a `Status`, or `None` if `value` is not a valid variant.
                    pub fn from_i32(value: i32) -> ::core::option::Option<Status> {
                        match value {
                            0 => ::core::option::Option::Some(Status::Rejected),
                            1 => ::core::option::Option::Some(Status::Complete),
                            2 => ::core::option::Option::Some(Status::Failure),
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
                            2 => ::core::result::Result::Ok(Status::Failure),
                            _ => {
                                ::core::result::Result::Err(
                                    ::prost::UnknownEnumValue(value),
                                )
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
                            Self::Failure => "FAILURE",
                        }
                    }
                    /// Creates an enum from field names used in the ProtoBuf definition.
                    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                        match value {
                            "REJECTED" => Some(Self::Rejected),
                            "COMPLETE" => Some(Self::Complete),
                            "FAILURE" => Some(Self::Failure),
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
                use super::{
                    MulticastMessage, MulticastMessageKind, MulticastMessageStatus,
                };
                mod new {
                    use super::{
                        MulticastMessage, MulticastMessageKind, MulticastMessageStatus,
                    };
                    use crate::models::Ticket;
                    impl MulticastMessage {
                        /// Creates a new `MulticastMessage` with the given `id` and `kind`.
                        pub fn new(
                            task: &str,
                            ticket: &Ticket,
                            kind: MulticastMessageKind,
                            status: MulticastMessageStatus,
                        ) -> Self {
                            Self {
                                task: task.to_owned(),
                                ticket: ticket.to_owned(),
                                kind: kind.into(),
                                timestamp: Some(
                                    prost_types::Timestamp::from(std::time::SystemTime::now()),
                                ),
                                status: status.into(),
                            }
                        }
                        /// Creates a new `MulticastMessage` with the given `id` and `kind` set to `Ticket`,
                        /// and `status` set to `Complete`.
                        pub fn new_ticket_complete(task: &str, ticket: &Ticket) -> Self {
                            Self::new(
                                task,
                                ticket,
                                MulticastMessageKind::Ticket,
                                MulticastMessageStatus::Complete,
                            )
                        }
                        /// Creates a new `MulticastMessage` with the given `id` and `kind` set to `Ticket`,
                        /// and `status` set to `Rejected`.
                        pub fn new_ticket_rejected(task: &str, ticket: &Ticket) -> Self {
                            Self::new(
                                task,
                                ticket,
                                MulticastMessageKind::Ticket,
                                MulticastMessageStatus::Rejected,
                            )
                        }
                    }
                }
                mod status {
                    use super::MulticastMessageStatus;
                    impl MulticastMessageStatus {
                        /// `true` if the status is considered finished, and no further processing is
                        /// expected.
                        ///
                        /// [`Failure`](MulticastMessageStatus::Failure) is not considered finished, as it
                        /// indicates an unexpected error that requires retrying.
                        pub fn is_finished(&self) -> bool {
                            match self {
                                Self::Complete | Self::Rejected => true,
                                _ => false,
                            }
                        }
                    }
                }
            }
        }
        pub use proto::*;
        mod query {
            /// [`QueryType`] is a trait that defines the methods that a query type must implement.
            ///
            /// This allows the designer to customise the query parameters to their needs, while
            /// maintaining a standardised interface for the waiter to know certain information about
            /// the query.
            pub trait QueryType: serde::de::DeserializeOwned + serde::Serialize + Send + Sync {
                /// Get the timeout for the query.
                ///
                /// This is used to determine how long the waiter should wait for a response
                /// before issuing a [`http::StatusCode::REQUEST_TIMEOUT`] response.
                ///
                /// While a [`None`] value is allowed, it is strongly recommended to enforce a
                /// [`Some<Duration>`] value to prevent the waiter from waiting indefinitely.
                fn get_timeout(&self) -> Option<tokio::time::Duration>;
                /// A parameter to determine if the query is asynchronous.
                ///
                /// Defaults to a function that always returns `false`.
                fn is_async(&self) -> bool {
                    false
                }
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
            pub struct OutputResponse<'o, O>
            where
                O: serde::Serialize,
            {
                pub ticket: Ticket,
                pub metadata: ResponseMetadata,
                pub output: &'o O,
            }
            #[automatically_derived]
            impl<'o, O: ::core::fmt::Debug> ::core::fmt::Debug for OutputResponse<'o, O>
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
            impl<'o, O: ::core::clone::Clone> ::core::clone::Clone
            for OutputResponse<'o, O>
            where
                O: serde::Serialize,
            {
                #[inline]
                fn clone(&self) -> OutputResponse<'o, O> {
                    OutputResponse {
                        ticket: ::core::clone::Clone::clone(&self.ticket),
                        metadata: ::core::clone::Clone::clone(&self.metadata),
                        output: ::core::clone::Clone::clone(&self.output),
                    }
                }
            }
            #[automatically_derived]
            impl<'o, O> ::core::marker::StructuralPartialEq for OutputResponse<'o, O>
            where
                O: serde::Serialize,
            {}
            #[automatically_derived]
            impl<'o, O: ::core::cmp::PartialEq> ::core::cmp::PartialEq
            for OutputResponse<'o, O>
            where
                O: serde::Serialize,
            {
                #[inline]
                fn eq(&self, other: &OutputResponse<'o, O>) -> bool {
                    self.ticket == other.ticket && self.metadata == other.metadata
                        && self.output == other.output
                }
            }
            #[automatically_derived]
            impl<'o, O: ::core::cmp::Eq> ::core::cmp::Eq for OutputResponse<'o, O>
            where
                O: serde::Serialize,
            {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<Ticket>;
                    let _: ::core::cmp::AssertParamIsEq<ResponseMetadata>;
                    let _: ::core::cmp::AssertParamIsEq<&'o O>;
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'o, O> _serde::Serialize for OutputResponse<'o, O>
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
            impl<'o, O> OutputResponse<'o, O>
            where
                O: serde::Serialize,
            {
                /// Create a new [`OutputResponse`] instance.
                pub fn new(
                    ticket: Ticket,
                    output: &'o O,
                    start_time: &tokio::time::Instant,
                ) -> Self {
                    Self {
                        ticket,
                        metadata: ResponseMetadata::new(start_time),
                        output,
                    }
                }
            }
            impl<O> IntoResponse for OutputResponse<'_, O>
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
    /// The default partition key (Primary Key) to use with the DynamoDB Table.
    ///
    /// This must be set to match the table's partition key.
    const DEFAULT_DYNAMODB_PARTITION_KEY: &str = "identifier";
    /// The default TTL for the results in seconds.
    const DEFAULT_RESULT_TTL: f32 = 7200.;
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
        /// The AWS DynamoDB table to use.
        #[arg(long, default_value = None)]
        pub dynamodb_table: Option<String>,
        /// The partition key to use with the DynamoDB table.
        #[arg(
            long,
            default_value = DEFAULT_DYNAMODB_PARTITION_KEY,
            alias = "dynamodb_primary_key"
        )]
        pub dynamodb_partition_key: String,
        /// The number of seconds to keep the results in the DynamoDB table before it can
        /// get purged by AWS.
        #[arg(long, default_value_t = DEFAULT_RESULT_TTL)]
        pub result_ttl: f32,
        /// The maximum time a ticket can be processed before it is killed by the
        /// HTTP server.
        #[arg(long, default_value = None)]
        pub max_execution_time: Option<f32>,
        /// The AWS SQS queue URL to use.
        ///
        /// The AWS user must have the necessary permissions to send and receive messages
        /// from this queue
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
                dynamodb_partition_key: __clap_arg_matches
                    .remove_one::<String>("dynamodb_partition_key")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: dynamodb_partition_key",
                    ))?,
                result_ttl: __clap_arg_matches
                    .remove_one::<f32>("result_ttl")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: result_ttl",
                    ))?,
                max_execution_time: __clap_arg_matches
                    .remove_one::<f32>("max_execution_time"),
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
            if __clap_arg_matches.contains_id("dynamodb_partition_key") {
                #[allow(non_snake_case)]
                let dynamodb_partition_key = &mut self.dynamodb_partition_key;
                *dynamodb_partition_key = __clap_arg_matches
                    .remove_one::<String>("dynamodb_partition_key")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: dynamodb_partition_key",
                    ))?;
            }
            if __clap_arg_matches.contains_id("result_ttl") {
                #[allow(non_snake_case)]
                let result_ttl = &mut self.result_ttl;
                *result_ttl = __clap_arg_matches
                    .remove_one::<f32>("result_ttl")
                    .ok_or_else(|| clap::Error::raw(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "The following required argument was not provided: result_ttl",
                    ))?;
            }
            if __clap_arg_matches.contains_id("max_execution_time") {
                #[allow(non_snake_case)]
                let max_execution_time = &mut self.max_execution_time;
                *max_execution_time = __clap_arg_matches
                    .remove_one::<f32>("max_execution_time");
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
                                let members: [clap::Id; 11usize] = [
                                    clap::Id::from("host"),
                                    clap::Id::from("port"),
                                    clap::Id::from("multicast_host"),
                                    clap::Id::from("multicast_port"),
                                    clap::Id::from("baristas"),
                                    clap::Id::from("max_tickets"),
                                    clap::Id::from("dynamodb_table"),
                                    clap::Id::from("dynamodb_partition_key"),
                                    clap::Id::from("result_ttl"),
                                    clap::Id::from("max_execution_time"),
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
                        let arg = arg
                            .help("The AWS DynamoDB table to use")
                            .long_help(None)
                            .long("dynamodb-table")
                            .default_value(None);
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("dynamodb_partition_key")
                            .value_name("DYNAMODB_PARTITION_KEY")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The partition key to use with the DynamoDB table")
                            .long_help(None)
                            .long("dynamodb-partition-key")
                            .default_value(DEFAULT_DYNAMODB_PARTITION_KEY)
                            .alias("dynamodb_primary_key");
                        let arg = arg;
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("result_ttl")
                            .value_name("RESULT_TTL")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    f32,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "The number of seconds to keep the results in the DynamoDB table before it can get purged by AWS",
                            )
                            .long_help(None)
                            .long("result-ttl")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: f32 = DEFAULT_RESULT_TTL;
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
                        let arg = clap::Arg::new("max_execution_time")
                            .value_name("MAX_EXECUTION_TIME")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    f32,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "The maximum time a ticket can be processed before it is killed by the HTTP server",
                            )
                            .long_help(None)
                            .long("max-execution-time")
                            .default_value(None);
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
                        let arg = arg
                            .help("The AWS SQS queue URL to use")
                            .long_help(
                                "The AWS SQS queue URL to use.\n\nThe AWS user must have the necessary permissions to send and receive messages from this queue",
                            )
                            .long("sqs-queue")
                            .default_value(None);
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
                                let members: [clap::Id; 11usize] = [
                                    clap::Id::from("host"),
                                    clap::Id::from("port"),
                                    clap::Id::from("multicast_host"),
                                    clap::Id::from("multicast_port"),
                                    clap::Id::from("baristas"),
                                    clap::Id::from("max_tickets"),
                                    clap::Id::from("dynamodb_table"),
                                    clap::Id::from("dynamodb_partition_key"),
                                    clap::Id::from("result_ttl"),
                                    clap::Id::from("max_execution_time"),
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
                        let arg = arg
                            .help("The AWS DynamoDB table to use")
                            .long_help(None)
                            .long("dynamodb-table")
                            .default_value(None);
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("dynamodb_partition_key")
                            .value_name("DYNAMODB_PARTITION_KEY")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    String,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help("The partition key to use with the DynamoDB table")
                            .long_help(None)
                            .long("dynamodb-partition-key")
                            .default_value(DEFAULT_DYNAMODB_PARTITION_KEY)
                            .alias("dynamodb_primary_key");
                        let arg = arg.required(false);
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("result_ttl")
                            .value_name("RESULT_TTL")
                            .required(false && clap::ArgAction::Set.takes_values())
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    f32,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "The number of seconds to keep the results in the DynamoDB table before it can get purged by AWS",
                            )
                            .long_help(None)
                            .long("result-ttl")
                            .default_value({
                                static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                                let s = DEFAULT_VALUE
                                    .get_or_init(|| {
                                        let val: f32 = DEFAULT_RESULT_TTL;
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
                        let arg = clap::Arg::new("max_execution_time")
                            .value_name("MAX_EXECUTION_TIME")
                            .value_parser({
                                use ::clap_builder::builder::impl_prelude::*;
                                let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                    f32,
                                >::new();
                                (&&&&&&auto).value_parser()
                            })
                            .action(clap::ArgAction::Set);
                        let arg = arg
                            .help(
                                "The maximum time a ticket can be processed before it is killed by the HTTP server",
                            )
                            .long_help(None)
                            .long("max-execution-time")
                            .default_value(None);
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
                        let arg = arg
                            .help("The AWS SQS queue URL to use")
                            .long_help(
                                "The AWS SQS queue URL to use.\n\nThe AWS user must have the necessary permissions to send and receive messages from this queue",
                            )
                            .long("sqs-queue")
                            .default_value(None);
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
                "dynamodb_partition_key",
                "result_ttl",
                "max_execution_time",
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
                &self.dynamodb_partition_key,
                &self.result_ttl,
                &self.max_execution_time,
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
                && self.dynamodb_partition_key == other.dynamodb_partition_key
                && self.result_ttl == other.result_ttl
                && self.max_execution_time == other.max_execution_time
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
                dynamodb_partition_key: DEFAULT_DYNAMODB_PARTITION_KEY.to_owned(),
                result_ttl: DEFAULT_RESULT_TTL,
                max_execution_time: None,
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
                    message: ::alloc::__export::must_use({
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
                    message: ::alloc::__export::must_use({
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
        /// Builder pattern - change the DynamoDB configuration.
        pub fn with_dynamodb_table(mut self, table: &str) -> Self {
            self.dynamodb_table = Some(table.to_owned());
            self
        }
        /// Builder pattern - change the DynamoDB partition key.
        pub fn with_dynamodb_partition_key(mut self, key: &str) -> Self {
            self.dynamodb_partition_key = key.to_owned();
            self
        }
        /// Builder pattern - change the result TTL.
        pub fn with_result_ttl(mut self, ttl: f32) -> Self {
            self.result_ttl = ttl;
            self
        }
        /// Builder pattern - change the SQS queue URL.
        pub fn with_sqs_queue(mut self, queue: String) -> Self {
            self.sqs_queue = Some(queue);
            self
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
        /// Get the DynamoDB TTL in [`tokio::time::Duration`] format.
        pub fn dynamodb_ttl(&self) -> tokio::time::Duration {
            tokio::time::Duration::from_secs_f32(self.result_ttl)
        }
        /// Get the maximum execution time in [`tokio::time::Duration`] format.
        pub fn max_execution_time(&self) -> Option<tokio::time::Duration> {
            self.max_execution_time
                .map(|secs| tokio::time::Duration::from_secs_f32(secs))
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
