use std::future::Future;

use crate::{ActorFactory, VirtualActor};

/// Factory trait for virtual actor
pub trait VirtualActorFactory: ActorFactory
where
    Self::Actor: VirtualActor,
{
    /// Creates new virtual actor
    fn create_actor(
        &self,
        id: &<Self::Actor as VirtualActor>::ActorId,
    ) -> impl Future<Output = Self::Actor>;
}
