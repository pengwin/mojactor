use std::sync::Arc;

use std::future::Future;

use virtual_actor::{Actor, ActorContext, ActorFactory};

use crate::{address::ActorHandle, context::ActorContextFactory, Addr};

use super::{error::ActorTaskError, mailbox::Mailbox};

/// Actor loop trait
pub trait ActorLoop<A, AF, CF>: Send + Sync + Clone + 'static
where
    A: Actor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: ActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    /// Message processing loop for an actor.
    fn actor_loop(
        self,
        mailbox: Mailbox<A>,
        actor_factory: Arc<AF>,
        context_factory: Arc<CF>,
        handle: Arc<ActorHandle<A>>,
    ) -> impl Future<Output = Result<(), ActorTaskError>>;
}
