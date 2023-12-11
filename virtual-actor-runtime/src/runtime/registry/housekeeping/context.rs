use virtual_actor::{ActorContext, CancellationToken, VirtualActor};

use crate::Addr;

use super::HousekeepingActor;

pub struct CancellationTokenStub;

impl CancellationToken for CancellationTokenStub {
    async fn cancelled(&self) {}
}

pub struct HousekeepingContext<A: VirtualActor> {
    pub(super) addr: Addr<HousekeepingActor<A>>,
}

impl<A: VirtualActor> Clone for HousekeepingContext<A> {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr.create_clone(),
        }
    }
}

impl<A: VirtualActor> ActorContext<HousekeepingActor<A>> for HousekeepingContext<A> {
    type Addr = Addr<HousekeepingActor<A>>;

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
