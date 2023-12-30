use std::marker::PhantomData;

use crate::{ActorFactory, LocalActorConstructor, LocalActorFactory};

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
