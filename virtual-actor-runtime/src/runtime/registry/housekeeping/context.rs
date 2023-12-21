use virtual_actor::{ActorAddr, ActorContext, CancellationToken, VirtualActor};

use crate::{LocalAddr, WeakLocalAddr};

use super::HousekeepingActor;

pub struct CancellationTokenStub;

impl CancellationToken for CancellationTokenStub {
    async fn cancelled(&self) {}
}

pub struct HousekeepingContext<A: VirtualActor> {
    pub(super) weak_addr: WeakLocalAddr<HousekeepingActor<A>>,
}

impl<A: VirtualActor> Clone for HousekeepingContext<A> {
    fn clone(&self) -> Self {
        Self {
            weak_addr: self.weak_addr.clone(),
        }
    }
}

impl<A: VirtualActor> ActorContext<HousekeepingActor<A>> for HousekeepingContext<A> {
    type Addr = LocalAddr<HousekeepingActor<A>>;

    type CancellationToken = CancellationTokenStub;

    fn self_addr(&self) -> &<Self::Addr as ActorAddr<HousekeepingActor<A>>>::WeakRef {
        &self.weak_addr
    }

    fn stop(&self) {
        unimplemented!()
    }

    fn cancellation_token(&self) -> &Self::CancellationToken {
        &CancellationTokenStub
    }
}
