//! `ActorAddr` implementation

use virtual_actor::{
    actor::{Actor, ActorAddr},
    message::{Message, MessageEnvelopeFactory, MessageHandler},
};

use crate::{errors::WaitError, GracefulShutdown};

use super::{actor_handle::ActorHandle, weak_local_addr::WeakLocalAddr};

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
    ) -> Result<(), super::errors::ActorStartError> {
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
    type Error = super::errors::LocalAddrError;

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
