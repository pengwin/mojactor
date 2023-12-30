//! Mailbox for actor

use tokio_util::sync::CancellationToken;
use virtual_actor::actor::Actor;
use virtual_actor::message::MailboxPreferences;

use crate::messaging::Mailbox as BaseMailbox;
use crate::messaging::MessageDispatcher;
use crate::utils::atomic_counter::AtomicCounter;

/// Mailbox for actor
pub struct Mailbox<A: Actor> {
    inner: BaseMailbox<A::MessagesEnvelope>,
}

impl<A: Actor> Mailbox<A> {
    /// Creates new mailbox
    pub fn new(
        preferences: &MailboxPreferences,
        mailbox_cancellation: &CancellationToken,
        dispatched_msg_counter: &AtomicCounter,
    ) -> (MessageDispatcher<A>, Self) {
        let (mailbox_sender, inner) = BaseMailbox::new(preferences, mailbox_cancellation);
        let dispatcher = MessageDispatcher::new(mailbox_sender, dispatched_msg_counter.clone());
        (dispatcher, Self { inner })
    }

    /// Receive message from mailbox
    pub async fn recv(&mut self, ct: &CancellationToken) -> Option<A::MessagesEnvelope> {
        self.inner.recv(ct).await
    }
}
