use crate::errors::{CoffeeMachineError, CoffeeShopError};

/// A type alias for the result of processing a ticket.
pub type ProcessResult<O> = Result<O, CoffeeShopError>;

/// A type alias for the result of calling the coffee machine.
pub type MachineResult<O> = Result<O, CoffeeMachineError>;
