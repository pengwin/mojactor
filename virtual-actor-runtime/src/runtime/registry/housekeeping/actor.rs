use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use virtual_actor::{Actor, LocalActor, MessageHandler, VirtualActor};

use crate::address::ActorHandle;

use super::{context::HousekeepingContext, envelope::InnerMessageEnvelope};

pub struct HousekeepingActor<A: VirtualActor> {
    pub(super) cache: Arc<DashMap<A::ActorId, Arc<ActorHandle<A>>>>,
    pub(super) interval: Duration,
    pub(super) actor_idle_timeout: Duration,
}

impl<A: VirtualActor> Actor for HousekeepingActor<A> {
    type ActorContext = HousekeepingContext<A>;

    type MessagesEnvelope = InnerMessageEnvelope;

    fn name() -> virtual_actor::names::ActorName {
        stringify!(HousekeepingActor)
    }

    async fn handle_envelope(
        &mut self,
        envelope: Self::MessagesEnvelope,
        ctx: &Self::ActorContext,
    ) -> Result<(), virtual_actor::ResponderError> {
        match envelope {
            InnerMessageEnvelope::GarbageCollectActors(msg) => {
                self.handle(msg, ctx).await;
                Ok(())
            }
        }
    }
}

impl<A: VirtualActor> LocalActor for HousekeepingActor<A> {}
