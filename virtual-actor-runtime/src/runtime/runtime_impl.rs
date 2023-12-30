use std::sync::Arc;

use virtual_actor::{
    actor::{Actor, ActorFactory},
    local_actor::{DefaultLocalActorFactory, LocalActor, LocalActorConstructor, LocalActorFactory},
    virtual_actor::{
        DefaultVirtualActorFactory, VirtualActor, VirtualActorConstructor, VirtualActorFactory,
    },
};

use crate::{
    address::VirtualAddr,
    errors::WaitError,
    executor::{LocalExecutor, LocalExecutorError},
    ExecutorHandle, ExecutorPreferences, GracefulShutdown, LocalAddr, RuntimeContext,
    RuntimeContextFactory, TokioRuntimePreferences,
};

use super::{
    registry::{ActivateActorError, ActorRegistry},
    runtime_preferences::RuntimePreferences,
};

/// Virtual actor runtime
pub struct Runtime {
    preferences: Arc<RuntimePreferences>,
    registry: ActorRegistry,
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
        let housekeeping_executor = LocalExecutor::new(&ExecutorPreferences {
            tokio_runtime_preferences: TokioRuntimePreferences {
                enable_io: false,
                enable_time: true,
            },
            thread_name: "housekeeping-executor".to_string(),
            ..Default::default()
        })?;

        let registry = ActorRegistry::new(housekeeping_executor.handle());
        Ok(Self {
            preferences: Arc::new(preferences),
            registry,
            executors: vec![housekeeping_executor],
        })
    }

    /// Creates executor based on `tokio::LocalSet`
    ///
    /// # Errors
    ///
    /// Returns error if was not able to create executor
    pub fn create_executor(&mut self) -> Result<ExecutorHandle, LocalExecutorError> {
        let thread_name = format!("local-executor-{}", self.executors.len());
        self.create_executor_with_preferences(&ExecutorPreferences {
            thread_name,
            ..Default::default()
        })
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
    pub async fn spawn_local<A>(
        &self,
        executor: &ExecutorHandle,
    ) -> Result<LocalAddr<A>, LocalExecutorError>
    where
        A: LocalActor + LocalActorConstructor,
        A: Actor<ActorContext = RuntimeContext<A>>,
    {
        let actor_factory = Arc::new(DefaultLocalActorFactory::default());
        self.spawn_local_with_factory(&actor_factory, executor)
            .await
    }

    /// Spawns local actor on executor
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub async fn spawn_local_with_factory<AF>(
        &self,
        actor_factory: &Arc<AF>,
        executor: &ExecutorHandle,
    ) -> Result<LocalAddr<<AF as ActorFactory>::Actor>, LocalExecutorError>
    where
        <AF as ActorFactory>::Actor:
            Actor<ActorContext = RuntimeContext<<AF as ActorFactory>::Actor>>,
        AF: LocalActorFactory,
        <AF as ActorFactory>::Actor: LocalActor,
    {
        let context_factory = Arc::new(RuntimeContextFactory::<<AF as ActorFactory>::Actor>::new(
            self.registry.weak_ref(),
        ));
        let handle = executor
            .spawn_local_actor(
                actor_factory,
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
    pub fn register_actor<A>(&self, executor: &ExecutorHandle) -> Result<(), LocalExecutorError>
    where
        A: VirtualActor + VirtualActorConstructor,
        A: Actor<ActorContext = RuntimeContext<A>>,
    {
        let factory = DefaultVirtualActorFactory::<A>::default();
        self.register_actor_with_factory(factory, executor)
    }

    /// Registers virtual actor
    ///
    /// # Errors
    ///
    /// Returns error if was not able to register actor
    pub fn register_actor_with_factory<AF>(
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
            self.registry.weak_ref(),
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
    async fn graceful_shutdown(mut self, timeout: std::time::Duration) -> Result<(), WaitError> {
        for executor in self.executors.drain(..) {
            executor.graceful_shutdown(timeout).await?;
        }

        println!("Runtime is stopped");
        Ok(())
    }
}
