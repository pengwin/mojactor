use super::super::local_actor::LocalActor;
use crate::messaging::{MailboxDispatcher, MailboxError};

/// Dispatcher for `LocalSpawner`
#[derive(Clone)]
pub struct SpawnerDispatcher {
    sender: MailboxDispatcher<Box<dyn LocalActor>>,
}

impl SpawnerDispatcher {
    /// Creates new dispatcher
    pub fn new(inner: MailboxDispatcher<Box<dyn LocalActor>>) -> Self {
        Self { sender: inner }
    }

    /// Send message to mailbox
    pub fn send(&self, actor: Box<dyn LocalActor>) -> Result<(), MailboxError> {
        self.sender
            .try_send(actor)
            .map_err(MailboxError::from_try_send_error)
    }

    /// Check if mailbox is closed
    /// Used in unit test
    #[allow(dead_code)]
    pub(crate) fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}
