use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use virtual_actor::{actor::ActorFactory, local_actor::LocalActorFactory, virtual_actor::VirtualActor};

use crate::runtime::{
    registry::actors_cache::ActorsCache, runtime_preferences::RuntimePreferences,
};

use super::{actor_counters_map::ActorCountersMap, HousekeepingActor};

pub struct HousekeepingActorFactory<A: VirtualActor> {
    graceful_cancellation: CancellationToken,
    cache: ActorsCache<A>,
    preferences: Arc<RuntimePreferences>,
}

impl<A: VirtualActor> HousekeepingActorFactory<A> {
    pub fn new(
        graceful_cancellation: CancellationToken,
        cache: ActorsCache<A>,
        preferences: &Arc<RuntimePreferences>,
    ) -> Self {
        Self {
            graceful_cancellation,
            cache,
            preferences: preferences.clone(),
        }
    }
}

impl<A: VirtualActor> ActorFactory for HousekeepingActorFactory<A> {
    type Actor = HousekeepingActor<A>;
}

impl<A: VirtualActor> LocalActorFactory for HousekeepingActorFactory<A> {
    type Error = std::convert::Infallible;

    async fn create_actor(&self) -> Result<HousekeepingActor<A>, Self::Error> {
        Ok(HousekeepingActor {
            graceful_cancellation: self.graceful_cancellation.clone(),
            cache: self.cache.clone(),
            preferences: self.preferences.clone(),
            actor_counters: ActorCountersMap::new(),
        })
    }
}
