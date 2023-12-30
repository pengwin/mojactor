use std::{marker::PhantomData, sync::Arc};

use tokio::select;
use virtual_actor::{
    actor::{Actor, ActorContext, ActorFactory},
    local_actor::{LocalActor, LocalActorFactory},
};

use crate::{
    address::ActorHandle, context::ActorContextFactory, utils::notify_once::NotifyOnce, LocalAddr,
};

use super::{super::actor_loop::ActorLoop, super::error::ActorTaskError, super::mailbox::Mailbox};

pub struct LocalActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
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
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
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
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
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
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: LocalActorFactory + 'static,
    <AF as ActorFactory>::Actor: LocalActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    async fn actor_loop(
        self,
        mut mailbox: Mailbox<<AF as ActorFactory>::Actor>,
        actor_started: Arc<NotifyOnce>,
        actor_factory: Arc<AF>,
        context_factory: Arc<CF>,
        handle: ActorHandle<<AF as ActorFactory>::Actor>,
    ) -> Result<(), ActorTaskError> {
        let mut actor = actor_factory
            .create_actor()
            .await
            .map_err(ActorTaskError::actor_factory_error)?;

        let context = context_factory.create_context(&handle);
        let task_ct = handle.cancellation_token();

        actor_started.notify();

        while let Some(envelope) = mailbox.recv(task_ct).await {
            select! {
                biased;
                () = task_ct.cancelled() => Err(ActorTaskError::Cancelled),
                r = actor.handle_envelope(envelope, &context) => r.map_err(ActorTaskError::ResponderError),
            }?;
        }
        Ok(())
    }
}
