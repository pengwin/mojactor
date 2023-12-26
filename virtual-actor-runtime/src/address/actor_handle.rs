//! Actor handler implementation

use std::sync::{Arc, OnceLock, Weak};

use tokio::{select, sync::Notify};
use tokio_util::sync::CancellationToken;
use virtual_actor::{
    Actor, Message, MessageEnvelopeFactory, MessageHandler, MessageProcessingResult,
};

use crate::{
    executor::ActorTaskError,
    messaging::{DispatcherError, MessageDispatcher},
    utils::{atomic_counter::AtomicCounter, GracefulShutdown},
    utils::{
        notify_once::NotifyOnce,
        waiter::{waiter, WaitError},
    },
    LocalAddr,
};

use super::{
    actor_task_container::{ActorTaskContainer, ActorTaskContainerError},
    local_addr::LocalAddrError,
    ActorTask,
};

/// Actor start error
#[derive(thiserror::Error, Debug)]
pub enum ActorStartError {
    /// Start wait error
    #[error("Start wait error: {0:?}")]
    WaitError(#[from] WaitError),
    /// Actor task error
    #[error("ActorTaskError: {0:?}")]
    ActorTaskError(#[from] ActorTaskError),
    /// Unexpected state
    #[error("Unexpected state {0}")]
    UnexpectedState(String),
}

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
    /// Actor started
    actor_started: Arc<NotifyOnce>,
    /// Actor stopped
    actor_stopped: Arc<NotifyOnce>,
    /// Actor execution cancellation token
    execution_cancellation: CancellationToken,
    /// Message receiving cancellation token
    mailbox_cancellation: CancellationToken,
    /// Counter of messages dispatched to actor
    dispatched_msg_counter: AtomicCounter,
    /// Counter of messages processed by actor
    processed_msg_counter: AtomicCounter,
    /// Actor task
    actor_task: ActorTaskContainer,
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
        dispatched_msg_counter: AtomicCounter,
    ) -> Self {
        Self {
            inner: Arc::new(ActorInner {
                dispatcher,
                actor_started: Arc::new(NotifyOnce::new()),
                actor_stopped: Arc::new(NotifyOnce::new()),
                dispatcher_ready: Arc::new(Notify::new()),
                execution_cancellation,
                mailbox_cancellation,
                dispatched_msg_counter,
                processed_msg_counter: AtomicCounter::default(),
                actor_task: ActorTaskContainer::default(),
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

    /// Set actor task
    pub(crate) fn set_task(&self, task: ActorTask) -> Result<(), ActorTaskContainerError> {
        self.inner.actor_task.set(task)
    }

    pub(crate) fn processed_msg_counter(&self) -> &AtomicCounter {
        &self.inner.processed_msg_counter
    }

    pub(crate) fn dispatched_msg_counter(&self) -> &AtomicCounter {
        &self.inner.dispatched_msg_counter
    }

    /// Clone cancellation token
    pub(crate) fn cancellation_token(&self) -> &CancellationToken {
        &self.inner.execution_cancellation
    }

    /// Clone mailbox cancellation token
    pub(crate) fn mailbox_cancellation(&self) -> &CancellationToken {
        &self.inner.mailbox_cancellation
    }

    pub(crate) fn stop_notify(&self) -> &Arc<NotifyOnce> {
        &self.inner.actor_stopped
    }

    pub(crate) fn start_notify(&self) -> &Arc<NotifyOnce> {
        &self.inner.actor_started
    }

    /// Is finished
    pub fn is_finished(&self) -> bool {
        self.inner.actor_stopped.is_notified()
    }

    /// Wait for actor to be ready
    pub async fn wait_for_ready(
        &self,
        timeout: std::time::Duration,
    ) -> Result<(), ActorStartError> {
        if self.inner.actor_started.is_notified() {
            return Ok(());
        }

        self.wait_for_dispatcher(timeout).await?;
        self.wait_for_start(timeout).await
    }

    /// Wait for dispatcher to be set
    async fn wait_for_dispatcher(&self, timeout: std::time::Duration) -> Result<(), WaitError> {
        waiter(
            "wait_for_dispatcher",
            &self.inner.dispatcher_ready,
            timeout,
            Some(&self.inner.mailbox_cancellation),
        )
        .await
    }

    /// Wait for actor to be started
    async fn wait_for_start(&self, timeout: std::time::Duration) -> Result<(), ActorStartError> {
        let name = "wait_for_start".to_owned();
        let res = select! {
            biased;
            () = self.inner.mailbox_cancellation.cancelled() => Err(WaitError::Cancelled(name.clone())),
            () = self.inner.actor_started.wait_for_notify() => Ok(true),
            () = self.inner.actor_stopped.wait_for_notify() => Ok(false),
            () = tokio::time::sleep(timeout) => Err(WaitError::Timeout(name)),
        }?;

        if res {
            return Ok(());
        }

        let task_error = self
            .extract_task_error()
            .await
            .map_err(|e| ActorStartError::UnexpectedState(e.message))?;

        if let Some(error) = task_error {
            return Err(ActorStartError::ActorTaskError(error));
        }

        Err(ActorStartError::UnexpectedState(
            "Actor task is not set or doesn't have errors".to_owned(),
        ))
    }

    pub async fn extract_task_error(
        &self,
    ) -> Result<Option<ActorTaskError>, ActorTaskContainerError> {
        let task = self.inner.actor_task.take()?;

        if let Some(task) = task {
            return match task.await {
                Ok(res) => match res {
                    Ok(()) => Ok(None),
                    Err(e) => Ok(Some(e)),
                },
                Err(e) => Ok(Some(ActorTaskError::TaskJoinError(e))),
            };
        }

        Ok(None)
    }

    /// Impl for trait
    pub async fn send<M>(&self, msg: M) -> Result<M::Result, LocalAddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        <A as Actor>::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        if self.is_finished() {
            return Err(LocalAddrError::Stopped);
        }
        let dispatcher = self
            .inner
            .dispatcher
            .get()
            .ok_or(LocalAddrError::ActorNotReady)?;
        select! {
            biased;
            () = self.inner.mailbox_cancellation.cancelled() => Err(LocalAddrError::Stopped),
            res = dispatcher.send(msg) => Self::map_actor_response::<M>(res),
        }
    }

    /// Impl for trait
    pub fn dispatch<M>(&self, msg: M) -> Result<(), LocalAddrError>
    where
        M: Message,
        A: MessageHandler<M>,
        <A as Actor>::MessagesEnvelope: MessageEnvelopeFactory<A, M>,
    {
        if self.is_finished() {
            return Err(LocalAddrError::Stopped);
        }

        let dispatcher = self
            .inner
            .dispatcher
            .get()
            .ok_or(LocalAddrError::ActorNotReady)?;
        if self.inner.mailbox_cancellation.is_cancelled() {
            return Err(LocalAddrError::Stopped);
        }
        if self.inner.execution_cancellation.is_cancelled() {
            return Err(LocalAddrError::Stopped);
        }

        dispatcher
            .dispatch(msg)
            .map_err(LocalAddrError::DispatcherError)
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

    fn map_actor_response<M: Message>(
        res: Result<MessageProcessingResult<M>, DispatcherError>,
    ) -> Result<M::Result, LocalAddrError> {
        match res {
            Ok(r) => match r {
                Ok(res) => Ok(res),
                Err(err) => Err(LocalAddrError::MessageProcessingError(err)),
            },
            Err(err) => Err(LocalAddrError::DispatcherError(err)),
        }
    }
}
