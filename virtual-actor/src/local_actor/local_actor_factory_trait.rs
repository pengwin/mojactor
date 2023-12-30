use std::future::Future;

use super::LocalActor;
use crate::actor::ActorFactory;

/// Factory trait for local actors
pub trait LocalActorFactory: ActorFactory
where
    Self::Actor: LocalActor,
{
    /// Error type for error occurred during actor creation
    type Error: std::error::Error + Send + Sync;

    /// Creates new local actor
    fn create_actor(&self) -> impl Future<Output = Result<Self::Actor, Self::Error>>;
}
