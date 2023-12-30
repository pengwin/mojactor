use std::future::Future;

use super::VirtualActor;
use crate::actor::ActorFactory;

/// Factory trait for virtual actor
pub trait VirtualActorFactory: ActorFactory
where
    Self::Actor: VirtualActor,
{
    /// Error type for error occurred during actor creation
    type Error: std::error::Error + Send + Sync;

    /// Creates new virtual actor
    fn create_actor(
        &self,
        id: &<Self::Actor as VirtualActor>::ActorId,
    ) -> impl Future<Output = Result<Self::Actor, Self::Error>>;
}
