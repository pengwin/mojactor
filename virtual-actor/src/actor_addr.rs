//! Actor addr trait

use std::future::Future;

use crate::{Actor, Message, MessageEnvelopeFactory, MessageHandler};

/// Actor handler error
#[derive(thiserror::Error, Debug)]
pub enum AddrError {
    /// Dispatcher not set
    #[error("Actor not ready to receive messages")]
    ActorNotReady,
    /// Dispatcher not set
    #[error("Actor stopped")]
    Cancelled,
    /// Dispatcher error
    #[error("Dispatch error {0}")]
    DispatcherError(Box<dyn std::error::Error + Send + Sync>),
}

impl AddrError {
    /// Creates new `AddrError::DispatcherError`
    pub fn dispatcher_error<E>(e: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::DispatcherError(Box::new(e))
    }
}

/// Actor address
pub trait ActorAddr<A: Actor>: Send + Sync + Sized {
    /// Weak ref impl
    type WeakRef: WeakActorRef<A, Self>;

    /// Creates new actor address
    fn weak_ref(&self) -> Self::WeakRef;

    /// Sends message to actor and waits for response
    ///
    /// # Errors
    ///
    /// Returns `ActorAddrError::ActorNotReady` if dispatcher is not set
    /// Returns `ActorAddrError::DispatcherError` if dispatcher error occurred
    fn send<M>(&self, msg: M) -> impl Future<Output = Result<M::Result, AddrError>>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>;

    /// Sends message to actor without waiting for response
    ///
    /// # Errors
    ///
    /// Returns `ActorAddrError::ActorNotReady` if dispatcher is not set
    /// Returns `ActorAddrError::DispatcherError` if dispatcher error occurred
    fn dispatch<M>(&self, msg: M) -> Result<(), AddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>;
}

/// Weak actor address
pub trait WeakActorRef<A: Actor, AA: ActorAddr<A>>: Send + Sync + Clone {
    /// Attempts to upgrade `WeakActorRef` to `ActorAddr`
    fn upgrade(&self) -> Option<AA>;
}
