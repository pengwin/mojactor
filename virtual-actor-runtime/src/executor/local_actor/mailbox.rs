//! Mailbox for actor

use tokio::{
    select,
    sync::mpsc::{unbounded_channel, UnboundedReceiver},
};
use tokio_util::sync::CancellationToken;
use virtual_actor::Actor;

use crate::message_dispatcher::MessageDispatcher;

/// Mailbox for actor
pub struct Mailbox<A: Actor> {
    /// Mailbox channel
    receiver: UnboundedReceiver<A::MessagesEnvelope>,
    /// Message receiving cancellation token
    receiver_cancellation: CancellationToken,
    /// is closed
    closed: bool,
}

impl<A: Actor> Mailbox<A> {
    /// Creates new mailbox
    pub fn new(mailbox_cancellation: &CancellationToken) -> (MessageDispatcher<A>, Self) {
        let (mailbox_sender, mailbox) = unbounded_channel::<<A as Actor>::MessagesEnvelope>();
        let dispatcher = MessageDispatcher::new(mailbox_sender);
        (
            dispatcher,
            Self {
                closed: false,
                receiver: mailbox,
                receiver_cancellation: mailbox_cancellation.clone(),
            },
        )
    }

    /// Receive message from mailbox
    pub async fn recv(&mut self, ct: &CancellationToken) -> Option<A::MessagesEnvelope> {
        if !self.closed && self.receiver_cancellation.is_cancelled() {
            self.receiver.close();
            self.closed = true;
        }

        if self.closed {
            self.recv_with_ct(ct).await
        } else {
            self.recv_with_mailbox_ct(ct).await
        }
    }

    async fn recv_with_mailbox_ct(&mut self, ct: &CancellationToken) -> Option<A::MessagesEnvelope> {
        let mailbox_ct = self.receiver_cancellation.clone();
        let clone_ct = ct.clone();
        select! {
            () = mailbox_ct.cancelled() => {
                self.receiver.close();
                self.closed = true;
                self.recv_with_ct(&clone_ct).await
            },
            envelope = self.recv_with_ct(ct) => envelope,
        }
    }

    async fn recv_with_ct(&mut self, ct: &CancellationToken) -> Option<A::MessagesEnvelope> {
        select! {
            () = ct.cancelled() => None,
            envelope = self.receiver.recv() => envelope,
        }
    }
}
