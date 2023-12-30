use std::sync::Arc;

use virtual_actor::{
    actor::{Actor, ActorContext, ActorFactory},
    virtual_actor::{VirtualActor, VirtualActorFactory},
};

use crate::{
    address::ActorHandle, context::ActorContextFactory, executor::LocalExecutorError,
    ExecutorHandle, LocalAddr,
};

pub trait VirtualActorSpawner<A: VirtualActor>: Send + Sync {
    fn spawn_no_wait(&self, id: A::ActorId) -> Result<ActorHandle<A>, LocalExecutorError>;
}

pub struct VirtualActorRegistration<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    factory: Arc<AF>,
    executor: ExecutorHandle,
    context_factory: Arc<CF>,
}

impl<AF, CF> VirtualActorRegistration<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    pub fn new(factory: AF, context_factory: Arc<CF>, executor: &ExecutorHandle) -> Self {
        Self {
            factory: Arc::new(factory),
            executor: executor.clone(),
            context_factory,
        }
    }
}

impl<AF, CF> VirtualActorSpawner<<AF as ActorFactory>::Actor> for VirtualActorRegistration<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    fn spawn_no_wait(
        &self,
        id: <<AF as ActorFactory>::Actor as VirtualActor>::ActorId,
    ) -> Result<ActorHandle<<AF as ActorFactory>::Actor>, LocalExecutorError> {
        self.executor
            .spawn_virtual_actor(id, &self.factory, &self.context_factory)
    }
}
