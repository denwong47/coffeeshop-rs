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
