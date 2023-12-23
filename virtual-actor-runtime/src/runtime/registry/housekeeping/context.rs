use virtual_actor::{ActorAddr, ActorContext, VirtualActor};

use crate::{
    utils::cancellation_token_wrapper::CancellationTokenWrapper, LocalAddr, WeakLocalAddr,
};

use super::HousekeepingActor;

pub struct HousekeepingContext<A: VirtualActor> {
    pub(super) weak_addr: WeakLocalAddr<HousekeepingActor<A>>,
    /// Cancellation token
    pub(super) cancellation_token: CancellationTokenWrapper,
}

impl<A: VirtualActor> Clone for HousekeepingContext<A> {
    fn clone(&self) -> Self {
        Self {
            weak_addr: self.weak_addr.clone(),
            cancellation_token: self.cancellation_token.clone(),
        }
    }
}

impl<A: VirtualActor> ActorContext<HousekeepingActor<A>> for HousekeepingContext<A> {
    type Addr = LocalAddr<HousekeepingActor<A>>;

    type CancellationToken = CancellationTokenWrapper;

    fn self_addr(&self) -> &<Self::Addr as ActorAddr<HousekeepingActor<A>>>::WeakRef {
        &self.weak_addr
    }

    fn stop(&self) {
        unimplemented!()
    }

    fn cancellation_token(&self) -> &Self::CancellationToken {
        &self.cancellation_token
    }
}
