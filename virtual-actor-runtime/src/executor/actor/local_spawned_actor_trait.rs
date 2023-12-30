use super::errors::ActorSpawnError;

/// Local actor spawner trait
pub trait LocalSpawnedActor: Send {
    /// Spawn actor
    fn spawn(&self) -> Result<(), ActorSpawnError>;
}
