use std::{future::Future, marker::PhantomData};

use crate::{local_actor_trait::LocalActor, ActorFactory};

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

/// Constructor trait for local actors
pub trait LocalActorConstructor: LocalActor {
    /// Creates new local actor
    #[must_use]
    fn new() -> Self;
}

/// Default local actor factory for Actor which implements `LocalActorConstructor`
pub struct DefaultLocalActorFactory<A: LocalActorConstructor> {
    _a: PhantomData<fn(A) -> A>,
}

impl<A: LocalActorConstructor> Default for DefaultLocalActorFactory<A> {
    fn default() -> Self {
        Self { _a: PhantomData }
    }
}

impl<A: LocalActorConstructor> ActorFactory for DefaultLocalActorFactory<A> {
    type Actor = A;
}

impl<A: LocalActorConstructor> LocalActorFactory for DefaultLocalActorFactory<A> {
    type Error = std::convert::Infallible;

    async fn create_actor(&self) -> Result<Self::Actor, Self::Error> {
        Ok(A::new())
    }
}

impl<A: LocalActor + Default> LocalActorConstructor for A {
    fn new() -> Self {
        Self::default()
    }
}
