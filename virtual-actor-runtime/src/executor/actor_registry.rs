//! Implements registry to store spawned actors

use std::sync::Arc;

use dashmap::DashMap;

use super::local_actor::{ActorId, LocalActorHandle};

/// Registry to store spawned actors
#[derive(Clone)]
pub struct ActorRegistry {
    /// Map of spawned actors
    actors: Arc<DashMap<ActorId, LocalActorHandle>>,
}

impl ActorRegistry {
    /// Creates new actor registry
    pub fn new() -> Self {
        let registry = DashMap::new();
        Self {
            actors: Arc::new(registry),
        }
    }

    /// Adds actor to registry
    pub fn add_actor(&self, handle: LocalActorHandle) {
        self.actors.insert(handle.id(), handle);
    }

    /// Removes actor from registry
    pub fn remove_actor(&self, id: &ActorId) {
        self.actors.remove(id);
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.actors.is_empty()
    }
}
