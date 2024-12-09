use super::{MulticastMessage, MulticastMessageKind, MulticastMessageStatus};

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
            timestamp: Some(prost_types::Timestamp::from(std::time::SystemTime::now())),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_ticket_complete() {
        let task = "myTask";
        let id = "myId";
        let message = MulticastMessage::new_ticket_complete(task, id);
        assert_eq!(message.task, task);
        assert_eq!(message.id, id);
        assert_eq!(
            MulticastMessageKind::try_from(message.kind).unwrap(),
            MulticastMessageKind::Ticket
        );
        assert_eq!(
            MulticastMessageStatus::try_from(message.status).unwrap(),
            MulticastMessageStatus::Complete
        );
    }

    #[test]
    fn new_ticket_rejected() {
        let task = "myTask";
        let id = "myId";
        let message = MulticastMessage::new_ticket_rejected(task, id);
        assert_eq!(message.task, task);
        assert_eq!(message.id, id);
        assert_eq!(
            MulticastMessageKind::try_from(message.kind).unwrap(),
            MulticastMessageKind::Ticket
        );
        assert_eq!(
            MulticastMessageStatus::try_from(message.status).unwrap(),
            MulticastMessageStatus::Rejected
        );
    }
}
