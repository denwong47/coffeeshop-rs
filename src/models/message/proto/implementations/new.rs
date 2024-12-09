use super::{MulticastMessage, MulticastMessageKind, MulticastMessageStatus};

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
            timestamp: Some(prost_types::Timestamp::from(std::time::SystemTime::now())),
            status: status.into(),
        }
    }

    /// Creates a new `MulticastMessage` with the given `id` and `kind` set to `Ticket`,
    /// and `status` set to `Success`.
    pub fn new_ticket_complete(task: &str, ticket: &Ticket) -> Self {
        Self::new(
            task,
            ticket,
            MulticastMessageKind::Ticket,
            MulticastMessageStatus::Success,
        )
    }

    /// Creates a new `MulticastMessage` with the given `id` and `kind` set to `Ticket`,
    /// and `status` set to `Aborted`.
    pub fn new_ticket_rejected(task: &str, ticket: &Ticket) -> Self {
        Self::new(
            task,
            ticket,
            MulticastMessageKind::Ticket,
            MulticastMessageStatus::Aborted,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ticket_complete() {
        let task = "myTask";
        let ticket = "myId".to_owned();
        let message = MulticastMessage::new_ticket_complete(task, &ticket);
        assert_eq!(message.task, task);
        assert_eq!(message.ticket, ticket);
        assert_eq!(
            MulticastMessageKind::try_from(message.kind).unwrap(),
            MulticastMessageKind::Ticket
        );
        assert_eq!(
            MulticastMessageStatus::try_from(message.status).unwrap(),
            MulticastMessageStatus::Success
        );
    }

    #[test]
    fn new_ticket_rejected() {
        let task = "myTask";
        let ticket = "myId".to_owned();
        let message = MulticastMessage::new_ticket_rejected(task, &ticket);
        assert_eq!(message.task, task);
        assert_eq!(message.ticket, ticket);
        assert_eq!(
            MulticastMessageKind::try_from(message.kind).unwrap(),
            MulticastMessageKind::Ticket
        );
        assert_eq!(
            MulticastMessageStatus::try_from(message.status).unwrap(),
            MulticastMessageStatus::Aborted
        );
    }
}
