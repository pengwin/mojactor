//! Mailbox for actor

use tokio_util::sync::CancellationToken;
use virtual_actor::Actor;
use virtual_actor::MailboxPreferences;

use crate::messaging::Mailbox as BaseMailbox;
use crate::messaging::MessageDispatcher;

/// Mailbox for actor
pub struct Mailbox<A: Actor> {
    inner: BaseMailbox<A::MessagesEnvelope>,
}

impl<A: Actor> Mailbox<A> {
    /// Creates new mailbox
    pub fn new(
        preferences: &MailboxPreferences,
        mailbox_cancellation: &CancellationToken,
    ) -> (MessageDispatcher<A>, Self) {
        let (mailbox_sender, inner) = BaseMailbox::new(preferences, mailbox_cancellation);
        let dispatcher = MessageDispatcher::new(mailbox_sender);
        (dispatcher, Self { inner })
    }

    /// Receive message from mailbox
    pub async fn recv(&mut self, ct: &CancellationToken) -> Option<A::MessagesEnvelope> {
        self.inner.recv(ct).await
    }
}
