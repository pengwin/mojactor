use std::marker::PhantomData;

use virtual_actor::{Actor, VirtualActor};

use crate::{address::ActorHandle, context::ActorContextFactory, WeakLocalAddr};

use super::{context::HousekeepingContext, HousekeepingActor};

pub struct HousekeepingContextFactory<A: VirtualActor> {
    _a: PhantomData<fn(A) -> A>,
}

impl<A: VirtualActor> Default for HousekeepingContextFactory<A> {
    fn default() -> Self {
        Self { _a: PhantomData }
    }
}

impl<A: VirtualActor> ActorContextFactory<HousekeepingActor<A>> for HousekeepingContextFactory<A> {
    fn create_context(
        &self,
        handle: &ActorHandle<HousekeepingActor<A>>,
    ) -> <HousekeepingActor<A> as Actor>::ActorContext {
        HousekeepingContext {
            weak_addr: WeakLocalAddr::new(handle),
        }
    }
}
