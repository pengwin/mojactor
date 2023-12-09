use std::future::Future;

use crate::{local_actor_trait::LocalActor, ActorFactory};

/// Factory trait for local actors
pub trait LocalActorFactory: ActorFactory
where
    Self::Actor: LocalActor,
{
    /// Creates new local actor
    fn create_actor(&self) -> impl Future<Output = Self::Actor>;
}
