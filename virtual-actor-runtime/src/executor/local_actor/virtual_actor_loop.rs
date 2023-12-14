use std::{marker::PhantomData, sync::Arc};

use tokio::select;
use virtual_actor::{Actor, ActorContext, ActorFactory, VirtualActor, VirtualActorFactory};

use crate::utils::atomic_timestamp::AtomicTimestamp;
use crate::{address::ActorHandle, context::ActorContextFactory, LocalAddr};

use super::mailbox::Mailbox;
use super::{actor_loop::ActorLoop, error::ActorTaskError};

pub struct VirtualActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    actor_id: <<AF as ActorFactory>::Actor as VirtualActor>::ActorId,
    _af: PhantomData<fn(AF) -> AF>,
    _cf: PhantomData<fn(CF) -> CF>,
    last_processed_msg_timestamp: AtomicTimestamp,
}

impl<AF, CF> VirtualActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    pub fn new(
        actor_id: <<AF as ActorFactory>::Actor as VirtualActor>::ActorId,
        last_processed_msg_timestamp: &AtomicTimestamp,
    ) -> Self {
        let last_processed_msg_timestamp = last_processed_msg_timestamp.clone();
        last_processed_msg_timestamp.set_now();
        Self {
            actor_id,
            _af: PhantomData,
            _cf: PhantomData,
            last_processed_msg_timestamp,
        }
    }
}

impl<AF, CF> Clone for VirtualActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    fn clone(&self) -> Self {
        Self {
            actor_id: self.actor_id.clone(),
            _af: PhantomData,
            _cf: PhantomData,
            last_processed_msg_timestamp: self.last_processed_msg_timestamp.clone(),
        }
    }
}

impl<AF, CF> ActorLoop<AF, CF> for VirtualActorLoop<AF, CF>
where
    <<AF as ActorFactory>::Actor as Actor>::ActorContext:
        ActorContext<<AF as ActorFactory>::Actor, Addr = LocalAddr<<AF as ActorFactory>::Actor>>,
    AF: VirtualActorFactory + 'static,
    <AF as ActorFactory>::Actor: VirtualActor + 'static,
    CF: ActorContextFactory<<AF as ActorFactory>::Actor> + 'static,
{
    async fn actor_loop(
        self,
        mut mailbox: Mailbox<<AF as ActorFactory>::Actor>,
        actor_factory: Arc<AF>,
        context_factory: Arc<CF>,
        handle: ActorHandle<<AF as ActorFactory>::Actor>,
    ) -> Result<(), ActorTaskError> {
        let mut actor = actor_factory.create_actor(&self.actor_id).await;

        let context = context_factory.create_context(&handle);

        let task_ct = handle.cancellation_token();
        while let Some(envelope) = mailbox.recv(task_ct).await {
            select! {
                r = actor.handle_envelope(envelope, &context) => r.map_err(ActorTaskError::ResponderError),
                () = task_ct.cancelled() => Err(ActorTaskError::Cancelled),
            }?;
            self.last_processed_msg_timestamp.set_now();
        }
        //println!("Actor {id} is finished", id = self.actor_id);
        Ok(())
    }
}
