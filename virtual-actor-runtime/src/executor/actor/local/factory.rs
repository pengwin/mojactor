use std::sync::{Arc, OnceLock};

use tokio_util::sync::CancellationToken;
use virtual_actor::{
    actor::{Actor, ActorContext, ActorFactory},
    local_actor::{LocalActor, LocalActorFactory},
};

use crate::{
    address::ActorHandle, context::ActorContextFactory, utils::atomic_counter::AtomicCounter,
    LocalAddr,
};

use super::{
    super::{
        local_spawned_actor_impl::LocalSpawnedActorImpl,
        local_spawned_actor_trait::LocalSpawnedActor,
    },
    actor_loop::LocalActorLoop,
};

/// Creates new local actor
pub fn create_local_actor<AF, CF>(
    actor_factory: &Arc<AF>,
    context_factory: &Arc<CF>,
    execution_cancellation: CancellationToken,
    mailbox_cancellation: CancellationToken,
) -> (
    Box<dyn LocalSpawnedActor>,
    ActorHandle<<AF as ActorFactory>::Actor>,
)
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: LocalActorFactory + 'static,
    <AF as ActorFactory>::Actor: LocalActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    let dispatcher_ref = Arc::new(OnceLock::new());
    let dispatched_msg_counter = AtomicCounter::default();
    let handle = ActorHandle::new(
        dispatcher_ref,
        execution_cancellation,
        mailbox_cancellation,
        dispatched_msg_counter.clone(),
    );
    let actor_loop = LocalActorLoop::default();
    let spawner = LocalSpawnedActorImpl::new(
        actor_factory,
        context_factory,
        &handle,
        actor_loop,
        dispatched_msg_counter,
    );

    (Box::new(spawner), handle)
}
