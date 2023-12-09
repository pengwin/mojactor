use std::{marker::PhantomData, sync::Arc};

use tokio::select;
use virtual_actor::{Actor, ActorContext, ActorFactory, LocalActor, LocalActorFactory};

use crate::{address::ActorHandle, context::ActorContextFactory, Addr};

use super::{actor_loop::ActorLoop, error::ActorTaskError, mailbox::Mailbox};

pub struct LocalActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: LocalActorFactory + 'static,
    <AF as ActorFactory>::Actor: LocalActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    _af: PhantomData<fn(AF) -> AF>,
    _cf: PhantomData<fn(CF) -> CF>,
}

impl<AF, CF> Default for LocalActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: LocalActorFactory + 'static,
    <AF as ActorFactory>::Actor: LocalActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    fn default() -> Self {
        Self {
            _af: PhantomData,
            _cf: PhantomData,
        }
    }
}

impl<AF, CF> Clone for LocalActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: LocalActorFactory + 'static,
    <AF as ActorFactory>::Actor: LocalActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl<AF, CF> ActorLoop<AF, CF> for LocalActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = Addr<<AF as ActorFactory>::Actor>>,
    AF: LocalActorFactory + 'static,
    <AF as ActorFactory>::Actor: LocalActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    async fn actor_loop(
        self,
        mut mailbox: Mailbox<<AF as ActorFactory>::Actor>,
        actor_factory: Arc<AF>,
        context_factory: Arc<CF>,
        handle: Arc<ActorHandle<<AF as ActorFactory>::Actor>>,
    ) -> Result<(), ActorTaskError> {
        let mut actor = actor_factory.create_actor().await;

        let context = context_factory.create_context(&handle);

        let task_ct = handle.cancellation_token();
        while let Some(envelope) = mailbox.recv(task_ct).await {
            select! {
                r = actor.handle_envelope(envelope, &context) => r.map_err(ActorTaskError::ResponderError),
                () = task_ct.cancelled() => Err(ActorTaskError::Cancelled),
            }?;
        }
        Ok(())
    }
}
