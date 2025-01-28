mod base;
pub use base::Chain;

mod errors;
pub use errors::*;

mod iter;
pub use iter::IterChain;

mod segment;
pub use segment::ChainSegment;

#[cfg(test)]
mod tests;
