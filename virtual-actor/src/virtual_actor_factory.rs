use std::{future::Future, marker::PhantomData};

use crate::{ActorFactory, VirtualActor};

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

/// Constructor trait for virtual actors
pub trait VirtualActorConstructor: VirtualActor {
    /// Creates new virtual actor
    #[must_use]
    fn new(id: &Self::ActorId) -> Self;
}

/// Default virtual actor factory for Actor which implements `VirtualActorConstructor`
pub struct DefaultVirtualActorFactory<A: VirtualActorConstructor> {
    _a: PhantomData<fn(A) -> A>,
}

impl<A: VirtualActorConstructor> Default for DefaultVirtualActorFactory<A> {
    fn default() -> Self {
        Self { _a: PhantomData }
    }
}

impl<A: VirtualActorConstructor> ActorFactory for DefaultVirtualActorFactory<A> {
    type Actor = A;
}

impl<A: VirtualActorConstructor> VirtualActorFactory for DefaultVirtualActorFactory<A> {
    type Error = std::convert::Infallible;

    async fn create_actor(&self, id: &A::ActorId) -> Result<Self::Actor, Self::Error> {
        Ok(A::new(id))
    }
}
