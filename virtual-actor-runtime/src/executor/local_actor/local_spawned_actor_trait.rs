use crate::executor::actor_registry::ActorRegistry;

/// Local actor spawner trait
pub trait LocalSpawnedActor: Send {
    /// Spawn actor
    fn spawn(&self, actor_registry: &ActorRegistry);
}
