use super::MulticastMessageStatus;

impl MulticastMessageStatus {
    /// `true` if the status is considered finished, and no further processing is
    /// expected.
    ///
    /// [`Failure`](MulticastMessageStatus::Failure) is not considered finished, as it
    /// indicates an unexpected error that requires retrying.
    pub fn is_finished(&self) -> bool {
        matches!(self, Self::Complete | Self::Rejected)
    }
}
