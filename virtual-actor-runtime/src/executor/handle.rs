use std::{sync::Arc, time::Duration};

use tokio_util::sync::CancellationToken;
use virtual_actor::{
    actor::{Actor, ActorContext, ActorFactory},
    local_actor::{LocalActor, LocalActorFactory},
    virtual_actor::{VirtualActor, VirtualActorFactory},
};

use crate::{address::ActorHandle, context::ActorContextFactory, LocalAddr};

use super::{
    actor::{self, LocalSpawnedActor},
    error::LocalExecutorError,
    spawner::SpawnerDispatcher,
};

/// Handle to executor
#[derive(Clone)]
pub struct Handle {
    inner: Arc<InnerHandle>,
}

struct InnerHandle {
    /// Spawner dispatcher
    spawner_dispatcher: SpawnerDispatcher,
    /// Cancellation actor execution
    executor_cancellation: CancellationToken,
    /// Cancellation actor message processing
    mailbox_cancellation: CancellationToken,
}

impl Handle {
    /// Creates new handle
    pub(crate) fn new(
        spawner_dispatcher: SpawnerDispatcher,
        executor_cancellation: CancellationToken,
        mailbox_cancellation: CancellationToken,
    ) -> Self {
        Self {
            inner: Arc::new(InnerHandle {
                spawner_dispatcher,
                executor_cancellation,
                mailbox_cancellation,
            }),
        }
    }

    /// Accessor to cancellation token for actor execution
    pub(crate) fn executor_cancellation(&self) -> &CancellationToken {
        &self.inner.executor_cancellation
    }

    /// Accessor to cancellation token for actor message processing
    pub(crate) fn mailbox_cancellation(&self) -> &CancellationToken {
        &self.inner.mailbox_cancellation
    }

    /// Spawns local actor on thread
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub async fn spawn_local_actor<AF, CF>(
        &self,
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
        timeout: Duration,
    ) -> Result<ActorHandle<<AF as ActorFactory>::Actor>, LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext: ActorContext<
            <AF as ActorFactory>::Actor,
            Addr = LocalAddr<<AF as ActorFactory>::Actor>,
        >,
        AF: LocalActorFactory + 'static,
        <AF as ActorFactory>::Actor: LocalActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        self.spawn_actor(
            actor_factory,
            context_factory,
            actor::create_local_actor,
            timeout,
        )
        .await
    }

    /// Spawns local actor on thread
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub(crate) fn spawn_local_actor_no_wait<AF, CF>(
        &self,
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
    ) -> Result<ActorHandle<<AF as ActorFactory>::Actor>, LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext: ActorContext<
            <AF as ActorFactory>::Actor,
            Addr = LocalAddr<<AF as ActorFactory>::Actor>,
        >,
        AF: LocalActorFactory + 'static,
        <AF as ActorFactory>::Actor: LocalActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        self.spawn_actor_no_wait(actor_factory, context_factory, actor::create_local_actor)
    }

    /// Spawns virtual local actor on thread without waiting for dispatcher to be set
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub fn spawn_virtual_actor<AF, CF>(
        &self,
        actor_id: <<AF as ActorFactory>::Actor as VirtualActor>::ActorId,
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
    ) -> Result<ActorHandle<<AF as ActorFactory>::Actor>, LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext: ActorContext<
            <AF as ActorFactory>::Actor,
            Addr = LocalAddr<<AF as ActorFactory>::Actor>,
        >,
        AF: VirtualActorFactory + 'static,
        <AF as ActorFactory>::Actor: VirtualActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        self.spawn_actor_no_wait(actor_factory, context_factory, |af, cf, ct, m_ct| {
            actor::create_virtual_actor(actor_id, af, cf, ct, m_ct)
        })
    }

    /// Spawns actor on thread, without waiting for dispatcher to be set
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    fn spawn_actor_no_wait<AF, CF, F>(
        &self,
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
        spawner: F,
    ) -> Result<ActorHandle<<AF as ActorFactory>::Actor>, LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext: ActorContext<
            <AF as ActorFactory>::Actor,
            Addr = LocalAddr<<AF as ActorFactory>::Actor>,
        >,
        AF: ActorFactory + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
        F: FnOnce(
            &Arc<AF>,
            &Arc<CF>,
            CancellationToken,
            CancellationToken,
        ) -> (
            Box<dyn LocalSpawnedActor>,
            ActorHandle<<AF as ActorFactory>::Actor>,
        ),
    {
        let execution_ct = self.inner.executor_cancellation.child_token();
        let mailbox_ct = self.inner.mailbox_cancellation.child_token();
        let (local_actor, handle) =
            spawner(actor_factory, context_factory, execution_ct, mailbox_ct);

        self.inner
            .spawner_dispatcher
            .send(local_actor)
            .map_err(|e| LocalExecutorError::SpawnerSendError(format!("{e:?}")))?;

        Ok(handle)
    }

    async fn spawn_actor<AF, CF, F>(
        &self,
        actor_factory: &Arc<AF>,
        context_factory: &Arc<CF>,
        spawner: F,
        timeout: Duration,
    ) -> Result<ActorHandle<<AF as ActorFactory>::Actor>, LocalExecutorError>
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext: ActorContext<
            <AF as ActorFactory>::Actor,
            Addr = LocalAddr<<AF as ActorFactory>::Actor>,
        >,
        AF: ActorFactory + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
        F: FnOnce(
            &Arc<AF>,
            &Arc<CF>,
            CancellationToken,
            CancellationToken,
        ) -> (
            Box<dyn LocalSpawnedActor>,
            ActorHandle<<AF as ActorFactory>::Actor>,
        ),
    {
        let handle = self.spawn_actor_no_wait(actor_factory, context_factory, spawner)?;
        handle.wait_for_ready(timeout).await?;
        Ok(handle)
    }
}
