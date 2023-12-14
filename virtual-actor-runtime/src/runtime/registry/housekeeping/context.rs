use virtual_actor::{ActorContext, CancellationToken, VirtualActor};

use crate::LocalAddr;

use super::HousekeepingActor;

pub struct CancellationTokenStub;

impl CancellationToken for CancellationTokenStub {
    async fn cancelled(&self) {}
}

pub struct HousekeepingContext<A: VirtualActor> {
    pub(super) addr: LocalAddr<HousekeepingActor<A>>,
}

impl<A: VirtualActor> Clone for HousekeepingContext<A> {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr.clone(),
        }
    }
}

impl<A: VirtualActor> ActorContext<HousekeepingActor<A>> for HousekeepingContext<A> {
    type Addr = LocalAddr<HousekeepingActor<A>>;

    type CancellationToken = CancellationTokenStub;

    fn self_addr(&self) -> &Self::Addr {
        &self.addr
    }

    fn stop(&self) {
        unimplemented!()
    }

    fn cancellation_token(&self) -> &Self::CancellationToken {
        &CancellationTokenStub
    }
}
