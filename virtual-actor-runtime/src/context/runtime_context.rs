//! Runtime context for actor.

use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, ActorContext};

use crate::address::Addr;

/// Runtime context for actor.
pub struct RuntimeContext<A: Actor> {
    /// Actor address
    self_addr: Addr<A>,
    /// Cancellation token
    cancellation_token: CancellationToken,
}

impl<A: Actor> RuntimeContext<A> {
    pub(crate) fn new(self_addr: Addr<A>, cancellation_token: CancellationToken) -> Self {
        Self {
            self_addr,
            cancellation_token,
        }
    }
}

impl<A> Clone for RuntimeContext<A>
where
    A: Actor,
{
    fn clone(&self) -> Self {
        Self {
            self_addr: self.self_addr.create_clone(),
            cancellation_token: self.cancellation_token.clone(),
        }
    }
}

impl<A: Actor> ActorContext<A> for RuntimeContext<A> {
    type Addr = Addr<A>;

    fn self_addr(&self) -> &Self::Addr {
        &self.self_addr
    }

    fn stop(&self) {
        self.cancellation_token.cancel();
    }
}

impl<A: Actor> RuntimeContext<A> {
    /// Returns actor execution cancellation token
    #[must_use]
    pub fn cancellation_token(&self) -> &CancellationToken {
        &self.cancellation_token
    }
}
