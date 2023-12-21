use std::sync::Arc;

use virtual_actor::{ActorFactory, LocalActorFactory, VirtualActor};

use crate::runtime::{
    registry::actors_cache::ActorsCache, runtime_preferences::RuntimePreferences,
};

use super::{actor_counters_map::ActorCountersMap, HousekeepingActor};

pub struct HousekeepingActorFactory<A: VirtualActor> {
    cache: ActorsCache<A>,
    preferences: Arc<RuntimePreferences>,
}

impl<A: VirtualActor> HousekeepingActorFactory<A> {
    pub fn new(cache: ActorsCache<A>, preferences: &Arc<RuntimePreferences>) -> Self {
        Self {
            cache,
            preferences: preferences.clone(),
        }
    }
}

impl<A: VirtualActor> ActorFactory for HousekeepingActorFactory<A> {
    type Actor = HousekeepingActor<A>;
}

impl<A: VirtualActor> LocalActorFactory for HousekeepingActorFactory<A> {
    async fn create_actor(&self) -> HousekeepingActor<A> {
        HousekeepingActor {
            cache: self.cache.clone(),
            preferences: self.preferences.clone(),
            actor_counters: ActorCountersMap::new(),
        }
    }
}
