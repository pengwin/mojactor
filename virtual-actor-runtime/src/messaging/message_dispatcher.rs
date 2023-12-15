//! Implements message dispatcher for `Actor`

use std::sync::Arc;

use tokio::sync::mpsc::error::TrySendError;
use virtual_actor::{Actor, Message, MessageEnvelopeFactory, MessageHandler};

use crate::utils::atomic_counter::AtomicCounter;

use super::{mailbox::MailboxDispatcher, one_shot_responder::OneshotResponder, MailboxError};

/// Message dispatcher error
#[derive(thiserror::Error, Debug)]
pub enum DispatcherError {
    /// Mailbox send error
    #[error("Mailbox error: {0}")]
    MailBoxError(#[from] MailboxError),
    /// Response receiver error
    #[error("Response receiver error: {0}")]
    ResponseReceiverError(#[from] tokio::sync::oneshot::error::RecvError),
}

impl DispatcherError {
    pub fn from_try_send_error<T>(e: TrySendError<T>) -> Self {
        Self::MailBoxError(MailboxError::from_try_send_error(e))
    }
}

/// Message dispatcher for `Actor`
pub struct MessageDispatcher<A: Actor> {
    /// Sender for mailbox
    mailbox_sender: Arc<MailboxDispatcher<A::MessagesEnvelope>>,
    /// Counter of messages dispatched to actor
    dispatched_msg_counter: AtomicCounter,
}

impl<A: Actor> Clone for MessageDispatcher<A> {
    fn clone(&self) -> Self {
        Self {
            mailbox_sender: self.mailbox_sender.clone(),
            dispatched_msg_counter: self.dispatched_msg_counter.clone(),
        }
    }
}

impl<A: Actor> MessageDispatcher<A> {
    /// Creates new message dispatcher
    pub fn new(
        mailbox_sender: MailboxDispatcher<A::MessagesEnvelope>,
        dispatched_msg_counter: AtomicCounter,
    ) -> Self {
        Self {
            mailbox_sender: Arc::new(mailbox_sender),
            dispatched_msg_counter,
        }
    }
}

impl<A: Actor> MessageDispatcher<A> {
    /// Sends message to actor and waits for response
    pub async fn send<M>(&self, msg: M) -> Result<<M as Message>::Result, DispatcherError>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        let (responder, receiver) = OneshotResponder::new();
        let envelope = A::MessagesEnvelope::from_message(msg, Some(responder));
        self.mailbox_sender
            .try_send(envelope)
            .map_err(DispatcherError::from_try_send_error)?;

        self.dispatched_msg_counter.increment();

        let a = receiver.await?;

        Ok(a)
    }

    /// Sends message to actor without waiting for response
    pub fn dispatch<M>(&self, msg: M) -> Result<(), DispatcherError>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        let envelope = A::MessagesEnvelope::from_message(msg, None::<OneshotResponder<M>>);
        self.mailbox_sender
            .try_send(envelope)
            .map_err(DispatcherError::from_try_send_error)?;

        self.dispatched_msg_counter.increment();

        Ok(())
    }
}
