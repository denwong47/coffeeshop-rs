use super::CoffeeShopError;

use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http,
};

/// A trait to convert an error into a [`CoffeeShopError`], which implements
/// [`IntoResponse`](axum::response::IntoResponse).
pub trait IntoCoffeeShopError {
    /// Convert the error into a [`CoffeeShopError`].
    fn into_coffeeshop_error(self) -> CoffeeShopError;
}

impl IntoCoffeeShopError for CoffeeShopError {
    /// Allow [`CoffeeShopError`] to be converted into itself.
    fn into_coffeeshop_error(self) -> CoffeeShopError {
        self
    }
}

impl IntoCoffeeShopError for QueryRejection {
    /// Convert a [`QueryRejection`] into a [`CoffeeShopError`].
    fn into_coffeeshop_error(self) -> CoffeeShopError {
        match &self {
            QueryRejection::FailedToDeserializeQueryString(err) =>
            // TODO Placeholder error message for now;
            {
                CoffeeShopError::InvalidQueryOptions(err.body_text())
            }
            _ => CoffeeShopError::InvalidQueryOptions(format!(
                "Unknown query rejection: {:?}",
                self.body_text()
            )),
        }
    }
}

impl IntoCoffeeShopError for JsonRejection {
    /// Convert a [`JsonRejection`] into a [`CoffeeShopError`].
    fn into_coffeeshop_error(self) -> CoffeeShopError {
        match &self {
            JsonRejection::BytesRejection(err) =>
            // TODO Placeholder error message for now;
            {
                CoffeeShopError::InvalidPayload {
                    kind: "BytesRejection",
                    message: format!("Failed to buffer body: {}", err.body_text()),
                }
            }
            JsonRejection::JsonDataError(err) =>
            // TODO Placeholder error message for now;
            {
                CoffeeShopError::InvalidPayload {
                    kind: "JsonDataError",
                    message: err.body_text(),
                }
            }
            JsonRejection::JsonSyntaxError(err) =>
            // TODO Placeholder error message for now;
            {
                CoffeeShopError::MalformedJsonPayload(format!(
                    "Invalid JSON syntax: {}",
                    err.body_text()
                ))
            }
            JsonRejection::MissingJsonContentType(err) => CoffeeShopError::InvalidHeader {
                key: http::header::CONTENT_TYPE,
                message: format!(
                    "This endpoint only accepts JSON payload; {}. Please check your headers.",
                    err.body_text()
                ),
            },
            _ => CoffeeShopError::InvalidPayload {
                kind: "UnknownJsonRejection",
                message: format!("Unknown JSON rejection: {:?}", self.body_text()),
            },
        }
    }
}
