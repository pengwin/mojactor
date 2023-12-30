//! Actor addr trait

use std::future::Future;

use crate::message::{Message, MessageEnvelopeFactory, MessageHandler};

use super::{Actor, WeakActorAddr};

/// Actor address
pub trait ActorAddr<A: Actor>: Send + Sync + Sized {
    /// Error
    type Error: std::error::Error + 'static;

    /// Weak ref impl
    type WeakRef: WeakActorAddr<A>;

    /// Creates new actor address
    fn weak_ref(&self) -> Self::WeakRef;

    /// Sends message to actor and waits for response
    ///
    /// # Errors
    ///
    /// Returns `ActorAddrError::ActorNotReady` if dispatcher is not set
    /// Returns `ActorAddrError::DispatcherError` if dispatcher error occurred
    fn send<M>(&self, msg: M) -> impl Future<Output = Result<M::Result, Self::Error>>
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
    fn dispatch<M>(&self, msg: M) -> impl Future<Output = Result<(), Self::Error>>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>;
}
