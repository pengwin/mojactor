use std::sync::{Arc, OnceLock};

use tokio_util::sync::CancellationToken;
use virtual_actor::{
    ActorContext, LocalActor, LocalActorFactory, VirtualActor, VirtualActorFactory,
};

use crate::{address::ActorHandle, context::ActorContextFactory, Addr};

use super::{
    local_actor_loop::LocalActorLoop, local_spawned_actor_impl::LocalSpawnedActorImpl,
    local_spawned_actor_trait::LocalSpawnedActor, virtual_actor_loop::VirtualActorLoop,
};

/// Creates new local actor
pub fn create_local_actor<A, AF, CF>(
    actor_factory: &Arc<AF>,
    context_factory: &Arc<CF>,
    execution_cancellation: CancellationToken,
    mailbox_cancellation: CancellationToken,
) -> (Box<dyn LocalSpawnedActor>, Arc<ActorHandle<A>>)
where
    A: LocalActor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: LocalActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    let dispatcher_ref = Arc::new(OnceLock::new());
    let handle = Arc::new(ActorHandle::new(
        dispatcher_ref,
        execution_cancellation,
        mailbox_cancellation,
    ));
    let spawner = LocalSpawnedActorImpl::new(
        actor_factory,
        context_factory,
        &handle,
        LocalActorLoop::default(),
    );

    (Box::new(spawner), handle)
}

/// Creates new virtual actor
pub fn create_virtual_actor<A, AF, CF>(
    actor_id: A::ActorId,
    actor_factory: &Arc<AF>,
    context_factory: &Arc<CF>,
    execution_cancellation: CancellationToken,
    mailbox_cancellation: CancellationToken,
) -> (Box<dyn LocalSpawnedActor>, Arc<ActorHandle<A>>)
where
    A: VirtualActor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: VirtualActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    let dispatcher_ref = Arc::new(OnceLock::new());
    let handle = Arc::new(ActorHandle::new(
        dispatcher_ref,
        execution_cancellation,
        mailbox_cancellation,
    ));
    let spawner = LocalSpawnedActorImpl::new(
        actor_factory,
        context_factory,
        &handle,
        VirtualActorLoop::new(actor_id),
    );

    (Box::new(spawner), handle)
}
