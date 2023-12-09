use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use virtual_actor::{Actor, ActorContext, ActorFactory, VirtualActor, VirtualActorFactory};

use crate::{
    address::ActorHandle,
    context::ActorContextFactory,
    executor::{Handle, LocalExecutorError},
    Addr, LocalExecutor,
};

pub trait VirtualActorSpawner<A: VirtualActor>: Send + Sync {
    fn spawn_no_wait(&self, id: A::ActorId) -> Result<Arc<ActorHandle<A>>, LocalExecutorError>;
}

pub struct ActorActivator<A: VirtualActor> {
    registration: Box<dyn VirtualActorSpawner<A>>,
    cache: DashMap<A::ActorId, Arc<ActorHandle<A>>>,
}

impl<A: VirtualActor> ActorActivator<A> {
    pub fn new<AF, CF>(factory: AF, context_factory: Arc<CF>, executor: &LocalExecutor) -> Self
    where
        <<AF as ActorFactory>::Actor as Actor>::ActorContext:
            ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
        AF: VirtualActorFactory + 'static,
        AF: ActorFactory<Actor = A> + 'static,
        <AF as ActorFactory>::Actor: VirtualActor + 'static,
        CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
    {
        Self {
            registration: Box::new(VirtualActorRegistration::new(
                factory,
                context_factory,
                executor,
            )),
            cache: DashMap::new(),
        }
    }

    pub async fn spawn(
        &self,
        id: A::ActorId,
        wait_timeout: Duration,
    ) -> Result<Arc<ActorHandle<A>>, LocalExecutorError> {
        if let Some(handle) = self.cache.get(&id) {
            return Ok(handle.value().clone());
        }
        let handle = self.registration.spawn_no_wait(id.clone())?;
        handle.wait_for_dispatcher(wait_timeout).await?;
        self.cache.insert(id, handle.clone());
        Ok(handle)
    }
}

pub struct VirtualActorRegistration<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    factory: Arc<AF>,
    executor: Arc<Handle>,
    context_factory: Arc<CF>,
}

impl<AF, CF> VirtualActorRegistration<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    pub fn new(factory: AF, context_factory: Arc<CF>, executor: &LocalExecutor) -> Self {
        Self {
            factory: Arc::new(factory),
            executor: executor.clone_handle(),
            context_factory,
        }
    }
}

impl<AF, CF> VirtualActorSpawner<<AF as ActorFactory>::Actor> for VirtualActorRegistration<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    fn spawn_no_wait(
        &self,
        id: <<AF as ActorFactory>::Actor as VirtualActor>::ActorId,
    ) -> Result<Arc<ActorHandle<<AF as ActorFactory>::Actor>>, LocalExecutorError> {
        self.executor
            .spawn_virtual_actor(id, &self.factory, &self.context_factory)
    }
}
