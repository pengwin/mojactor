use std::sync::Arc;

use virtual_actor::{
    Actor, ActorFactory, LocalActor, LocalActorFactory, VirtualActor, VirtualActorFactory,
};

use crate::{
    address::ActorHandle, executor::LocalExecutorError, Addr, LocalExecutor, RuntimeContext,
    RuntimeContextFactory,
};

use super::registry::{ActivateActorError, ActorRegistry};

/// Virtual actor runtime
pub struct Runtime {
    registry: Arc<ActorRegistry>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            registry: Arc::new(ActorRegistry::new()),
        }
    }
}

impl Runtime {
    /// Spawns local actor on executor
    ///
    /// # Errors
    ///
    /// Returns error if executor thread is not started
    /// Returns error if spawner was not send
    pub async fn spawn_local<AF>(
        &self,
        factory: &Arc<AF>,
        executor: &LocalExecutor,
    ) -> Result<Arc<ActorHandle<<AF as ActorFactory>::Actor>>, LocalExecutorError>
    where
        <AF as ActorFactory>::Actor:
            Actor<ActorContext = RuntimeContext<<AF as ActorFactory>::Actor>>,
        AF: LocalActorFactory,
        <AF as ActorFactory>::Actor: LocalActor,
    {
        let context_factory = Arc::new(RuntimeContextFactory::<<AF as ActorFactory>::Actor>::new(
            self.registry.clone(),
        ));
        executor
            .handle()
            .spawn_local_actor(factory, &context_factory)
            .await
    }

    /// Registers virtual actor
    pub fn register_actor<AF>(&self, factory: AF, executor: &LocalExecutor)
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
            .register_actor(factory, context_factory, executor);
    }

    /// Spawns virtual actor on executor
    ///
    /// # Errors
    ///
    /// Returns error if actor is not started
    /// Returns error if actor type is not registered in runtime
    pub async fn spawn_virtual<A>(&self, id: A::ActorId) -> Result<Addr<A>, ActivateActorError>
    where
        A: VirtualActor + 'static,
    {
        self.registry.get_or_create(id).await
    }
}
