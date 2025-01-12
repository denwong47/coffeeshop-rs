use thiserror::Error;

/// Errors that can occur if an attachment fails.
///
/// All these errors will return the segment that it failed to attach, and
/// can be consumed to retry the operation.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum AttachmentError<T> {
    #[error("The segment {candidate:?} already has a matching key in the chain.")]
    KeyAlreadyExists {
        /// The segment that already exists in the chain.
        existing: T,
        candidate: T,
    },

    #[error("The current segment is not the tail of the chain.")]
    NotTail(T),
}

impl<T> AttachmentError<T> {
    /// Consumes the error and returns the segment that failed to attach.
    pub fn into_inner(self) -> T {
        macro_rules! into_inner {
            ($($variant:ident),+$(,)?) => {
                match self {
                    Self::KeyAlreadyExists { candidate, .. } => candidate,
                    $(Self::$variant(inner) => inner),+
                }
            };
        }

        into_inner!(NotTail,)
    }
}
