use std::sync::{Arc, OnceLock};

use tokio_util::sync::CancellationToken;
use virtual_actor::{
    Actor, ActorContext, ActorFactory, LocalActor, LocalActorFactory, Uuid, VirtualActor,
    VirtualActorFactory,
};

use crate::{
    address::ActorHandle, context::ActorContextFactory, utils::atomic_timestamp::AtomicTimestamp,
    LocalAddr,
};

use super::{
    local_actor_loop::LocalActorLoop, local_spawned_actor_impl::LocalSpawnedActorImpl,
    local_spawned_actor_trait::LocalSpawnedActor, virtual_actor_loop::VirtualActorLoop,
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
    let last_received_msg_timestamp = AtomicTimestamp::new();
    let handle = ActorHandle::new(
        dispatcher_ref,
        execution_cancellation,
        mailbox_cancellation,
        last_received_msg_timestamp.clone(),
    );
    let spawner = LocalSpawnedActorImpl::new(
        Uuid::new_v4(),
        actor_factory,
        context_factory,
        &handle,
        LocalActorLoop::default(),
        last_received_msg_timestamp,
    );

    (Box::new(spawner), handle)
}

/// Creates new virtual actor
pub fn create_virtual_actor<AF, CF>(
    actor_id: <<AF as ActorFactory>::Actor as VirtualActor>::ActorId,
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
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    let dispatcher_ref = Arc::new(OnceLock::new());
    let last_received_msg_timestamp = AtomicTimestamp::new();
    let handle = ActorHandle::new(
        dispatcher_ref,
        execution_cancellation,
        mailbox_cancellation,
        last_received_msg_timestamp.clone(),
    );
    let spawner = LocalSpawnedActorImpl::new(
        Uuid::new_v4(),
        actor_factory,
        context_factory,
        &handle,
        VirtualActorLoop::new(actor_id, handle.last_processed_msg_timestamp()),
        last_received_msg_timestamp,
    );

    (Box::new(spawner), handle)
}
