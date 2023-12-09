use std::sync::Arc;

use dashmap::DashMap;
use virtual_actor::Uuid;

use super::local_actor::ActorTaskError;

/// Spawned task join handle
pub type ActorTaskJoinHandle = tokio::task::JoinHandle<Result<(), ActorTaskError>>;

/// Spawned actor id
pub type SpawnedActorId = Uuid;

pub struct ActorTasksRegistry {
    /// Actor registry
    actors: DashMap<SpawnedActorId, ActorTaskJoinHandle>,
}

impl ActorTasksRegistry {
    /// Creates new `ActorTasksRegistry`
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            actors: DashMap::new(),
        })
    }

    /// Registers actor
    pub fn register_actor(&self, id: SpawnedActorId, handle: ActorTaskJoinHandle) {
        self.actors.insert(id, handle);
    }

    /// Unregisters actor
    pub fn unregister_actor(&self, id: &SpawnedActorId) {
        self.actors.remove(id);
    }

    /// Gets actors count
    pub fn count(&self) -> usize {
        self.actors.len()
    }
}
