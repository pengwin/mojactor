use crate::executor::actor_registry::ActorRegistry;

/// Local actor spawner trait
pub trait LocalActor: Send {
    /// Spawn actor
    fn spawn(&self, actor_registry: &ActorRegistry);
}
