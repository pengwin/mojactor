//! `ActorAddr` implementation

use virtual_actor::{
    Actor, ActorAddr, Message, MessageEnvelopeFactory, MessageHandler, MessageProcessingError,
};

use crate::{errors::WaitError, messaging::DispatcherError, GracefulShutdown};

use super::{
    actor_handle::{ActorHandle, ActorStartError},
    weak_local_addr::WeakLocalAddr,
};

/// Actor handler error
#[derive(thiserror::Error, Debug)]
pub enum LocalAddrError {
    /// Dispatcher not set
    #[error("Actor not ready to receive messages")]
    ActorNotReady,
    /// Dispatcher not set
    #[error("Actor stopped")]
    Stopped,
    /// Dispatcher error
    #[error("Dispatch error {0:?}")]
    DispatcherError(#[from] DispatcherError),
    /// Message processing error
    #[error("Message processing error {0:?}")]
    MessageProcessingError(#[from] MessageProcessingError),
}

/// Actor address
///
/// `LocalAddr` is a handle to an actor. It can be used to send messages to the actor.
/// If dropped, the actor will be cancelled from receiving new messages and later stopped
pub struct LocalAddr<A: Actor> {
    /// Actor handler
    handle: ActorHandle<A>,
}

impl<A: Actor> LocalAddr<A> {
    /// Creates new actor address
    pub(crate) fn new(handle: &ActorHandle<A>) -> Self {
        Self {
            handle: handle.clone(),
        }
    }

    /// Wait for actor to be ready
    ///
    /// # Errors
    ///
    /// Returns `WaitError::Timeout` if timeout is reached
    /// Returns `WaitError::Cancelled` if cancellation token is cancelled
    pub async fn wait_for_ready(
        &self,
        timeout: std::time::Duration,
    ) -> Result<(), ActorStartError> {
        self.handle.wait_for_ready(timeout).await
    }
}

impl<A: Actor> Drop for LocalAddr<A> {
    fn drop(&mut self) {
        // if all refs are dropped, we can cancel actor from receiving new messages
        self.handle.mailbox_cancellation().cancel();
    }
}

impl<A: Actor> ActorAddr<A> for LocalAddr<A> {
    type Error = LocalAddrError;

    type WeakRef = WeakLocalAddr<A>;

    async fn send<M>(&self, msg: M) -> Result<M::Result, Self::Error>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        self.handle.send(msg).await
    }

    async fn dispatch<M>(&self, msg: M) -> Result<(), Self::Error>
    where
        M: Message,
        A: MessageHandler<M>,
        A::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        self.handle.dispatch(msg)
    }

    fn weak_ref(&self) -> Self::WeakRef {
        WeakLocalAddr::new(&self.handle)
    }
}

impl<A: Actor> GracefulShutdown for LocalAddr<A> {
    async fn graceful_shutdown(self, timeout: std::time::Duration) -> Result<(), WaitError> {
        let graceful_shutdown_handle = self.handle.clone();
        graceful_shutdown_handle.graceful_shutdown(timeout).await
    }
}
