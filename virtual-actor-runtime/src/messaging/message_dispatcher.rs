//! Implements message dispatcher for `Actor`

use std::sync::Arc;

use virtual_actor::{Actor, Message, MessageEnvelopeFactory, MessageHandler};

use tokio::sync::mpsc::UnboundedSender;

use super::one_shot_responder::OneshotResponder;

/// Message dispatcher error
#[derive(thiserror::Error, Debug)]
pub enum DispatcherError {
    /// Mailbox send error
    #[error("Mailbox error: {0}")]
    MailBoxError(String),
    /// Response receiver error
    #[error("Response receiver error: {0}")]
    ResponseReceiverError(#[from] tokio::sync::oneshot::error::RecvError),
}

/// Message dispatcher for `Actor`
#[derive(Clone)]
pub struct MessageDispatcher<A: Actor> {
    /// Sender for mailbox
    mailbox_sender: Arc<UnboundedSender<A::MessagesEnvelope>>,
}

impl<A: Actor> MessageDispatcher<A> {
    /// Creates new message dispatcher
    pub fn new(mailbox_sender: UnboundedSender<A::MessagesEnvelope>) -> Self {
        Self {
            mailbox_sender: Arc::new(mailbox_sender),
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
            .send(envelope)
            .map_err(|e| DispatcherError::MailBoxError(format!("{e}")))?;

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
            .send(envelope)
            .map_err(|e| DispatcherError::MailBoxError(format!("{e}")))?;

        Ok(())
    }
}
