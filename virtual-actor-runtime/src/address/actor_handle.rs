//! Actor handler implementation

use std::sync::{Arc, OnceLock, Weak};

use tokio::{select, sync::Notify};
use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, AddrError, Message, MessageEnvelopeFactory, MessageHandler};

use crate::{
    messaging::MessageDispatcher,
    utils::{atomic_timestamp::AtomicTimestamp, GracefulShutdown},
    utils::{
        notify_once::NotifyOnce,
        waiter::{waiter, WaitError},
    },
    LocalAddr,
};

pub struct WeakActorHandle<A: Actor> {
    inner: Weak<ActorInner<A>>,
}

impl<A: Actor> WeakActorHandle<A> {
    pub fn upgrade(&self) -> Option<ActorHandle<A>> {
        self.inner.upgrade().map(|inner| ActorHandle { inner })
    }
}

impl<A: Actor> Clone for WeakActorHandle<A> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct ActorInner<A: Actor> {
    /// Actor dispatcher
    dispatcher: Arc<OnceLock<MessageDispatcher<A>>>,
    /// Dispatcher ready notify
    dispatcher_ready: Arc<Notify>,
    /// Actor stopped
    actor_stopped: Arc<NotifyOnce>,
    /// Actor execution cancellation token
    execution_cancellation: CancellationToken,
    /// Message receiving cancellation token
    mailbox_cancellation: CancellationToken,
    /// Timestamp of last processed message
    last_received_msg_timestamp: AtomicTimestamp,
    /// Timestamp of last processed message
    last_processed_msg_timestamp: AtomicTimestamp,
}

/// Actor handler
pub struct ActorHandle<A: Actor> {
    inner: Arc<ActorInner<A>>,
}

impl<A: Actor> Clone for ActorHandle<A> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<A: Actor> GracefulShutdown for ActorHandle<A> {
    async fn graceful_shutdown(self, timeout: std::time::Duration) -> Result<(), WaitError> {
        // first stop message receiving
        self.inner.mailbox_cancellation.cancel();
        let res = waiter(
            "actor_messaging_stopped",
            self.inner.actor_stopped.inner(),
            timeout,
            None,
        )
        .await;
        if let Err(WaitError::Timeout(_)) = res {
            // if timeout is reached
            // then cancel actor execution
            self.inner.execution_cancellation.cancel();
        } else {
            return res;
        }

        waiter(
            "actor_execution_stopped",
            self.inner.actor_stopped.inner(),
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
        last_received_msg_timestamp: AtomicTimestamp,
    ) -> Self {
        Self {
            inner: Arc::new(ActorInner {
                dispatcher,
                actor_stopped: Arc::new(NotifyOnce::new()),
                dispatcher_ready: Arc::new(Notify::new()),
                execution_cancellation,
                mailbox_cancellation,
                last_received_msg_timestamp,
                last_processed_msg_timestamp: AtomicTimestamp::new(),
            }),
        }
    }

    /// Set dispatcher
    pub(crate) fn set_dispatcher(
        &self,
        dispatcher: MessageDispatcher<A>,
    ) -> Result<(), &'static str> {
        match self.inner.dispatcher.set(dispatcher) {
            Ok(()) => {
                self.inner.dispatcher_ready.notify_one();
                Ok(())
            }
            Err(_) => Err("Dispatcher already set"),
        }
    }

    pub(crate) fn last_processed_msg_timestamp(&self) -> &AtomicTimestamp {
        &self.inner.last_processed_msg_timestamp
    }

    pub(crate) fn last_received_msg_timestamp(&self) -> &AtomicTimestamp {
        &self.inner.last_received_msg_timestamp
    }

    /// Clone cancellation token
    pub(crate) fn cancellation_token(&self) -> &CancellationToken {
        &self.inner.execution_cancellation
    }

    /// Clone mailbox cancellation token
    pub(crate) fn mailbox_cancellation(&self) -> &CancellationToken {
        &self.inner.mailbox_cancellation
    }

    /// Clone notify
    pub(crate) fn stop_notify(&self) -> &Arc<NotifyOnce> {
        &self.inner.actor_stopped
    }

    /// Is finished
    pub fn is_finished(&self) -> bool {
        self.inner.actor_stopped.is_notified()
    }

    /// Wait for dispatcher to be set
    pub async fn wait_for_dispatcher(&self, timeout: std::time::Duration) -> Result<(), WaitError> {
        waiter(
            "wait_for_dispatcher",
            &self.inner.dispatcher_ready,
            timeout,
            Some(&self.inner.execution_cancellation),
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
        if self.is_finished() {
            return Err(AddrError::Stopped);
        }
        let dispatcher = self
            .inner
            .dispatcher
            .get()
            .ok_or(AddrError::ActorNotReady)?;
        select! {
            res = dispatcher.send(msg) => res.map_err(AddrError::dispatcher_error),
            () = self.inner.execution_cancellation.cancelled() => Err(AddrError::Stopped),
        }
    }

    /// Impl for trait
    pub fn dispatch<M>(&self, msg: M) -> Result<(), AddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        <A as Actor>::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        if self.is_finished() {
            return Err(AddrError::Stopped);
        }

        let dispatcher = self
            .inner
            .dispatcher
            .get()
            .ok_or(AddrError::ActorNotReady)?;
        if self.inner.mailbox_cancellation.is_cancelled() {
            return Err(AddrError::Stopped);
        }
        if self.inner.execution_cancellation.is_cancelled() {
            return Err(AddrError::Stopped);
        }

        dispatcher
            .dispatch(msg)
            .map_err(AddrError::dispatcher_error)
    }

    /// Checks if actor is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.inner.execution_cancellation.is_cancelled()
            || self.inner.mailbox_cancellation.is_cancelled()
    }

    /// Get actor addr
    pub fn addr(&self) -> LocalAddr<A> {
        LocalAddr::new(self)
    }

    pub fn weak_ref(&self) -> WeakActorHandle<A> {
        WeakActorHandle {
            inner: Arc::downgrade(&self.inner),
        }
    }
}
