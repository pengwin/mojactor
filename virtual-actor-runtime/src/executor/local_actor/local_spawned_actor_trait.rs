use std::sync::Arc;

use crate::executor::actor_tasks_registry::{
    ActorTaskJoinHandle, ActorTasksRegistry, SpawnedActorId,
};

/// Local actor spawner trait
pub trait LocalSpawnedActor: Send {
    /// Gets actor id
    #[must_use]
    fn id(&self) -> SpawnedActorId;

    /// Spawn actor
    #[must_use]
    fn spawn(&self, registry: Arc<ActorTasksRegistry>) -> ActorTaskJoinHandle;
}
