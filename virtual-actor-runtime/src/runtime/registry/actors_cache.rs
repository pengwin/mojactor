use std::sync::Arc;

use dashmap::{mapref::multiple::RefMulti, DashMap};
use virtual_actor::VirtualActor;

use crate::address::ActorHandle;

pub struct ActorsCache<A: VirtualActor> {
    inner: Arc<Inner<A>>,
}

struct Inner<A: VirtualActor> {
    cache: DashMap<A::ActorId, ActorHandle<A>>,
}

impl<A: VirtualActor> ActorsCache<A> {
    pub fn new() -> Self {
        let inner = Inner {
            cache: DashMap::new(),
        };
        Self {
            inner: Arc::new(inner),
        }
    }

    pub fn get(&self, actor_id: &A::ActorId) -> Option<ActorHandle<A>> {
        self.inner
            .cache
            .get(actor_id)
            .map(|handle| handle.value().clone())
    }

    pub fn insert(&self, actor_id: A::ActorId, handle: ActorHandle<A>) {
        self.inner.cache.insert(actor_id, handle);
    }

    pub fn remove(&self, actor_id: &A::ActorId) -> Option<ActorHandle<A>> {
        self.inner.cache.remove(actor_id).map(|kv| kv.1)
    }

    pub fn iter(
        &self,
    ) -> impl Iterator<Item = RefMulti<'_, <A as VirtualActor>::ActorId, ActorHandle<A>>> {
        self.inner.cache.iter()
    }
}

impl<A: VirtualActor> Clone for ActorsCache<A> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
