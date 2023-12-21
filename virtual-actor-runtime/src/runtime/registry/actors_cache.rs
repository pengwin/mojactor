use std::sync::Arc;

use dashmap::{mapref::multiple::RefMulti, DashMap};
use virtual_actor::VirtualActor;

use crate::address::ActorHandle;

pub struct ActorsCache<A: VirtualActor> {
    cache: Arc<DashMap<A::ActorId, ActorHandle<A>>>,
}

impl<A: VirtualActor> ActorsCache<A> {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, actor_id: &A::ActorId) -> Option<ActorHandle<A>> {
        self.cache
            .get(actor_id)
            .map(|handle| handle.value().clone())
    }

    pub fn insert(&self, actor_id: A::ActorId, handle: ActorHandle<A>) {
        self.cache.insert(actor_id, handle);
    }

    pub fn remove(&self, actor_id: &A::ActorId) -> Option<ActorHandle<A>> {
        self.cache.remove(actor_id).map(|kv| kv.1)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = RefMulti<'_, <A as VirtualActor>::ActorId, ActorHandle<A>>> {
        self.cache.iter()
    }
}

impl<A: VirtualActor> Clone for ActorsCache<A> {
    fn clone(&self) -> Self {
        Self {
            cache: self.cache.clone(),
        }
    }
}
