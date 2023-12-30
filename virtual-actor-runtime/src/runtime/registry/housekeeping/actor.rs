use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use virtual_actor::{actor::Actor, local_actor::LocalActor, message::MessageHandler, virtual_actor::VirtualActor};

use crate::runtime::{
    registry::actors_cache::ActorsCache, runtime_preferences::RuntimePreferences,
};

use super::{
    actor_counters_map::ActorCountersMap, context::HousekeepingContext,
    envelope::InnerMessageEnvelope,
};

pub struct HousekeepingActor<A: VirtualActor> {
    pub(super) graceful_cancellation: CancellationToken,
    pub(super) cache: ActorsCache<A>,
    pub(super) preferences: Arc<RuntimePreferences>,
    pub(super) actor_counters: ActorCountersMap<A>,
}

impl<A: VirtualActor> Actor for HousekeepingActor<A> {
    type ActorContext = HousekeepingContext<A>;

    type MessagesEnvelope = InnerMessageEnvelope;

    fn name() -> virtual_actor::actor::ActorName {
        stringify!(HousekeepingActor)
    }

    async fn handle_envelope(
        &mut self,
        envelope: Self::MessagesEnvelope,
        ctx: &Self::ActorContext,
    ) -> Result<(), virtual_actor::errors::ResponderError> {
        match envelope {
            InnerMessageEnvelope::GarbageCollectActors(msg) => {
                self.handle(msg, ctx).await;
                Ok(())
            }
        }
    }
}

impl<A: VirtualActor> LocalActor for HousekeepingActor<A> {}
