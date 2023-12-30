use virtual_actor::{
    message::{MessageEnvelope, MessageEnvelopeFactory, Responder},
    virtual_actor::VirtualActor,
};

use super::{gc_actors::GarbageCollectActors, HousekeepingActor};

#[derive(Debug)]
pub enum InnerMessageEnvelope {
    GarbageCollectActors(GarbageCollectActors),
}

impl<A: VirtualActor> MessageEnvelopeFactory<HousekeepingActor<A>, GarbageCollectActors>
    for InnerMessageEnvelope
{
    fn from_message<R: Responder<GarbageCollectActors> + Sized + 'static>(
        msg: GarbageCollectActors,
        _responder: Option<R>,
    ) -> Self {
        Self::GarbageCollectActors(msg)
    }
}

impl<A: VirtualActor> MessageEnvelope<HousekeepingActor<A>> for InnerMessageEnvelope {}
