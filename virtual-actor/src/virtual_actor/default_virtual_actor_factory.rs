use std::marker::PhantomData;

use crate::{ActorFactory, VirtualActorConstructor, VirtualActorFactory};

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
