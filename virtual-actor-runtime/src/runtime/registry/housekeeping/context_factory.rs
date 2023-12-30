use std::marker::PhantomData;

use tokio_util::sync::CancellationToken;
use virtual_actor::{actor::Actor, virtual_actor::VirtualActor};

use crate::{
    address::ActorHandle, context::ActorContextFactory,
    utils::cancellation_token_wrapper::CancellationTokenWrapper, WeakLocalAddr,
};

use super::{context::HousekeepingContext, HousekeepingActor};

pub struct HousekeepingContextFactory<A: VirtualActor> {
    _a: PhantomData<fn(A) -> A>,
    cancellation_token: CancellationToken,
}

impl<A: VirtualActor> HousekeepingContextFactory<A> {
    pub fn new(cancellation_token: CancellationToken) -> Self {
        Self {
            _a: PhantomData,
            cancellation_token,
        }
    }
}

impl<A: VirtualActor> ActorContextFactory<HousekeepingActor<A>> for HousekeepingContextFactory<A> {
    fn create_context(
        &self,
        handle: &ActorHandle<HousekeepingActor<A>>,
    ) -> <HousekeepingActor<A> as Actor>::ActorContext {
        HousekeepingContext {
            weak_addr: WeakLocalAddr::new(handle),
            cancellation_token: CancellationTokenWrapper::new(self.cancellation_token.clone()),
        }
    }
}
