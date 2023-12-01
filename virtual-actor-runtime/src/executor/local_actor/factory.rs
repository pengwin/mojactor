use std::sync::{Arc, OnceLock};

use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, ActorContext, ActorFactory};

use crate::{actor_handle::ActorHandle, context_factory_trait::ActorContextFactory, Addr};

use super::{local_actor_impl::LocalActorImpl, local_actor_trait::LocalActor};

/// Creates new local actor
pub fn create<A, AF, CF>(
    actor_factory: &Arc<AF>,
    context_factory: &Arc<CF>,
    execution_cancellation: CancellationToken,
    mailbox_cancellation: CancellationToken,
) -> (Box<dyn LocalActor>, Arc<ActorHandle<A>>)
where
    A: Actor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: ActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    let dispatcher_ref = Arc::new(OnceLock::new());
    let handle = Arc::new(ActorHandle::new(
        dispatcher_ref,
        execution_cancellation,
        mailbox_cancellation,
    ));
    let spawner = LocalActorImpl::new(actor_factory, context_factory, &handle);

    (Box::new(spawner), handle)
}
