/// Models and helper functions relating to error handling.
pub mod handling;

mod shop_error;
pub use shop_error::CoffeeShopError;

mod generic_schema;
pub use generic_schema::ErrorSchema;

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

/// The error type for the Coffee Machine.
///
/// This is for downstream implementers to use as the error type for their Coffee Machine.
///
/// Currently this is an alias for [`ErrorSchema`].
pub type CoffeeMachineError = ErrorSchema;
