use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use virtual_actor::{ActorFactory, LocalActorFactory, VirtualActor};

use crate::{address::ActorHandle, runtime::runtime_preferences::RuntimePreferences};

use super::HousekeepingActor;

pub struct HousekeepingActorFactory<A: VirtualActor> {
    cache: Arc<DashMap<A::ActorId, Arc<ActorHandle<A>>>>,
    actor_idle_timeout: Duration,
    preferences: Arc<RuntimePreferences>,
}

impl<A: VirtualActor> HousekeepingActorFactory<A> {
    pub fn new(
        cache: Arc<DashMap<A::ActorId, Arc<ActorHandle<A>>>>,
        actor_idle_timeout: Duration,
        preferences: Arc<RuntimePreferences>,
    ) -> Self {
        Self {
            cache,
            actor_idle_timeout,
            preferences,
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
            interval: self.preferences.garbage_collect_interval,
            actor_idle_timeout: self.actor_idle_timeout,
        }
    }
}
