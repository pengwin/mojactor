//! Actor handler implementation

use std::sync::{Arc, OnceLock};

use tokio::{select, sync::Notify};
use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, AddrError, Message, MessageEnvelopeFactory, MessageHandler};

use crate::{
    messaging::MessageDispatcher,
    utils::waiter::{waiter, WaitError},
    utils::GracefulShutdown,
    Addr,
};

/// Actor handler
pub struct ActorHandle<A: Actor> {
    /// Actor dispatcher
    dispatcher: Arc<OnceLock<MessageDispatcher<A>>>,
    /// Dispatcher ready notify
    dispatcher_ready: Arc<Notify>,
    /// Actor stopped
    actor_stopped: Arc<Notify>,
    /// Actor execution cancellation token
    execution_cancellation: CancellationToken,
    /// Message receiving cancellation token
    mailbox_cancellation: CancellationToken,
}

impl<A: Actor> GracefulShutdown for ActorHandle<A> {
    async fn graceful_shutdown(&self, timeout: std::time::Duration) -> Result<(), WaitError> {
        // first stop message receiving
        self.mailbox_cancellation.cancel();
        let res = waiter(
            "actor_messaging_stopped",
            &self.actor_stopped,
            timeout,
            None,
        )
        .await;
        if let Err(WaitError::Timeout(_)) = res {
            // if timeout is reached
            // then cancel actor execution
            self.execution_cancellation.cancel();
        } else {
            return res;
        }

        waiter(
            "actor_execution_stopped",
            &self.actor_stopped,
            timeout,
            None,
        )
        .await
    }
}

impl<A: Actor> ActorHandle<A> {
    /// Creates new actor handler
    pub fn new(
        dispatcher: Arc<OnceLock<MessageDispatcher<A>>>,
        execution_cancellation: CancellationToken,
        mailbox_cancellation: CancellationToken,
    ) -> Self {
        Self {
            dispatcher,
            actor_stopped: Arc::new(Notify::new()),
            dispatcher_ready: Arc::new(Notify::new()),
            execution_cancellation,
            mailbox_cancellation,
        }
    }

    /// Set dispatcher
    pub fn set_dispatcher(&self, dispatcher: MessageDispatcher<A>) -> Result<(), &'static str> {
        match self.dispatcher.set(dispatcher) {
            Ok(()) => {
                self.dispatcher_ready.notify_one();
                Ok(())
            }
            Err(_) => Err("Dispatcher already set"),
        }
    }

    /// Clone cancellation token
    pub fn cancellation_token(&self) -> &CancellationToken {
        &self.execution_cancellation
    }

    /// Clone mailbox cancellation token
    pub fn mailbox_cancellation(&self) -> &CancellationToken {
        &self.mailbox_cancellation
    }

    /// Clone cancellation token
    pub fn stop_notify(&self) -> &Arc<Notify> {
        &self.actor_stopped
    }

    /// Wait for dispatcher to be set
    pub async fn wait_for_dispatcher(&self, timeout: std::time::Duration) -> Result<(), WaitError> {
        waiter(
            "wait_for_dispatcher",
            &self.dispatcher_ready,
            timeout,
            Some(&self.execution_cancellation),
        )
        .await
    }

    /// Impl for trait
    pub async fn send<M>(&self, msg: M) -> Result<M::Result, AddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        <A as Actor>::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        let dispatcher = self.dispatcher.get().ok_or(AddrError::ActorNotReady)?;
        select! {
            res = dispatcher.send(msg) => res.map_err(AddrError::dispatcher_error),
            () = self.execution_cancellation.cancelled() => Err(AddrError::Cancelled),
        }
    }

    /// Impl for trait
    pub fn dispatch<M>(&self, msg: M) -> Result<(), AddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        <A as Actor>::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        let dispatcher = self.dispatcher.get().ok_or(AddrError::ActorNotReady)?;
        if self.mailbox_cancellation.is_cancelled() {
            return Err(AddrError::Cancelled);
        }
        if self.execution_cancellation.is_cancelled() {
            return Err(AddrError::Cancelled);
        }

        dispatcher
            .dispatch(msg)
            .map_err(AddrError::dispatcher_error)
    }

    /// Checks if actor is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.execution_cancellation.is_cancelled() || self.mailbox_cancellation.is_cancelled()
    }

    /// Get actor addr
    pub fn addr(self: &Arc<Self>) -> Addr<A> {
        Addr::new(self)
    }
}
