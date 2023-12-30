use std::sync::Arc;

use std::future::Future;

use virtual_actor::actor::{Actor, ActorContext, ActorFactory};

use crate::{
    address::ActorHandle, context::ActorContextFactory, utils::notify_once::NotifyOnce, LocalAddr,
};

use super::{errors::ActorTaskError, mailbox::Mailbox};

/// Actor loop trait
pub trait ActorLoop<AF, CF>: Send + Sync + Clone + 'static
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: ActorFactory + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    /// Message processing loop for an actor.
    fn actor_loop(
        self,
        mailbox: Mailbox<<AF as ActorFactory>::Actor>,
        ctor_started: Arc<NotifyOnce>,
        actor_factory: Arc<AF>,
        context_factory: Arc<CF>,
        handle: ActorHandle<<AF as ActorFactory>::Actor>,
    ) -> impl Future<Output = Result<(), ActorTaskError>>;
}
