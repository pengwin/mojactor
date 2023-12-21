use std::sync::Arc;

use virtual_actor::{
    Actor, ActorFactory, LocalActor, LocalActorFactory, VirtualActor, VirtualActorFactory,
};

use crate::{
    address::VirtualAddr,
    executor::{LocalExecutor, LocalExecutorError},
    ExecutorHandle, ExecutorPreferences, GracefulShutdown, LocalAddr, RuntimeContext,
    RuntimeContextFactory,
};

use super::{
    registry::{ActivateActorError, ActorRegistry},
    runtime_preferences::RuntimePreferences,
};

/// Virtual actor runtime
pub struct Runtime {
    preferences: Arc<RuntimePreferences>,
    registry: Arc<ActorRegistry>,
    executors: Vec<LocalExecutor>,
}

impl Runtime {
    /// Creates new runtime
    ///
    /// # Errors
    ///
    /// Returns error if was not able to create actor registry
    pub fn new() -> Result<Self, LocalExecutorError> {
        Self::with_preferences(RuntimePreferences::default())
    }

    /// Creates new runtime with preferences
    ///
    /// # Errors
    ///
    /// Returns error if was not able to create actor registry
    pub fn with_preferences(preferences: RuntimePreferences) -> Result<Self, LocalExecutorError> {
        Ok(Self {
            preferences: Arc::new(preferences),
            registry: Arc::new(ActorRegistry::new()?),
            executors: Vec::new(),
        })
    }

    /// Creates executor based on `tokio::LocalSet`
    ///
    /// # Errors
    ///
    /// Returns error if was not able to create executor
    pub fn create_executor(&mut self) -> Result<ExecutorHandle, LocalExecutorError> {
        self.create_executor_with_preferences(&ExecutorPreferences::default())
    }

    /// Creates executor based on `tokio::LocalSet` with preferences
    ///
    /// # Errors
    ///
    /// Returns error if was not able to create executor
    pub fn create_executor_with_preferences(
        &mut self,
        preferences: &ExecutorPreferences,
    ) -> Result<ExecutorHandle, LocalExecutorError> {
        let executor = LocalExecutor::new(preferences)?;
        let handle = executor.handle().clone();
        self.executors.push(executor);
        Ok(handle)
    }

    /// Spawns local actor on executor
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub async fn spawn_local<AF>(
        &self,
        factory: &Arc<AF>,
        executor: &ExecutorHandle,
    ) -> Result<LocalAddr<<AF as ActorFactory>::Actor>, LocalExecutorError>
    where
        <AF as ActorFactory>::Actor:
            Actor<ActorContext = RuntimeContext<<AF as ActorFactory>::Actor>>,
        AF: LocalActorFactory,
        <AF as ActorFactory>::Actor: LocalActor,
    {
        let context_factory = Arc::new(RuntimeContextFactory::<<AF as ActorFactory>::Actor>::new(
            self.registry.clone(),
        ));
        let handle = executor
            .spawn_local_actor(
                factory,
                &context_factory,
                self.preferences.actor_activation_timeout,
            )
            .await?;
        Ok(handle.addr())
    }

    /// Registers virtual actor
    ///
    /// # Errors
    ///
    /// Returns error if was not able to register actor
    pub fn register_actor<AF>(
        &self,
        factory: AF,
        executor: &ExecutorHandle,
    ) -> Result<(), LocalExecutorError>
    where
        <AF as ActorFactory>::Actor:
            Actor<ActorContext = RuntimeContext<<AF as ActorFactory>::Actor>>,
        AF: VirtualActorFactory,
        <AF as ActorFactory>::Actor: VirtualActor,
    {
        let context_factory = Arc::new(RuntimeContextFactory::<<AF as ActorFactory>::Actor>::new(
            self.registry.clone(),
        ));
        self.registry
            .register_actor(factory, context_factory, executor, self.preferences.clone())
    }

    /// Spawns virtual actor on executor
    ///
    /// # Errors
    ///
    /// Returns error if actor is not started
    /// Returns error if actor type is not registered in runtime
    #[allow(clippy::unused_async)]
    pub async fn spawn_virtual<A>(
        &self,
        id: &A::ActorId,
    ) -> Result<VirtualAddr<A>, ActivateActorError>
    where
        A: VirtualActor + 'static,
    {
        self.registry.get_or_create(id)
    }
}

impl GracefulShutdown for Runtime {
    async fn graceful_shutdown(
        mut self,
        timeout: std::time::Duration,
    ) -> Result<(), crate::WaitError> {
        for executor in self.executors.drain(..) {
            executor.graceful_shutdown(timeout).await?;
        }

        Ok(())
    }
}
