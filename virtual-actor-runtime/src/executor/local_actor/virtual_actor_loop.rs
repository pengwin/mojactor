use std::{marker::PhantomData, sync::Arc};

use tokio::select;
use virtual_actor::{ActorContext, VirtualActor, VirtualActorFactory};

use crate::{address::ActorHandle, context::ActorContextFactory, Addr};

use super::{actor_loop::ActorLoop, error::ActorTaskError, mailbox::Mailbox};

pub struct VirtualActorLoop<A, AF, CF>
where
    A: VirtualActor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: VirtualActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    actor_id: A::ActorId,
    _a: PhantomData<fn(A) -> A>,
    _af: PhantomData<fn(AF) -> AF>,
    _cf: PhantomData<fn(CF) -> CF>,
}

impl<A, AF, CF> VirtualActorLoop<A, AF, CF>
where
    A: VirtualActor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: VirtualActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    pub fn new(actor_id: A::ActorId) -> Self {
        Self {
            actor_id,
            _a: PhantomData,
            _af: PhantomData,
            _cf: PhantomData,
        }
    }
}

impl<A, AF, CF> Clone for VirtualActorLoop<A, AF, CF>
where
    A: VirtualActor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: VirtualActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    fn clone(&self) -> Self {
        Self {
            actor_id: self.actor_id.clone(),
            _a: PhantomData,
            _af: PhantomData,
            _cf: PhantomData,
        }
    }
}

impl<A, AF, CF> ActorLoop<A, AF, CF> for VirtualActorLoop<A, AF, CF>
where
    A: VirtualActor + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
    AF: VirtualActorFactory<A> + 'static,
    CF: ActorContextFactory<A> + 'static,
{
    async fn actor_loop(
        self,
        mut mailbox: Mailbox<A>,
        actor_factory: Arc<AF>,
        context_factory: Arc<CF>,
        handle: Arc<ActorHandle<A>>,
    ) -> Result<(), ActorTaskError> {
        let mut actor = actor_factory.create_actor(&self.actor_id).await;

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
