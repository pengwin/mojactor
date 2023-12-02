//! Runtime context for actor.

use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, ActorContext};

use crate::address::Addr;

use super::cancellation_token_wrapper::CancellationTokenWrapper;

/// Runtime context for actor.
pub struct RuntimeContext<A: Actor> {
    /// Actor address
    self_addr: Addr<A>,
    /// Cancellation token
    cancellation_token: CancellationTokenWrapper,
    /// Mailbox cancellation token
    mailbox_cancellation_token: CancellationToken,
}

impl<A: Actor> RuntimeContext<A> {
    pub(crate) fn new(
        self_addr: Addr<A>,
        mailbox_cancellation_token: &CancellationToken,
        cancellation_token: &CancellationToken,
    ) -> Self {
        Self {
            self_addr,
            mailbox_cancellation_token: mailbox_cancellation_token.clone(),
            cancellation_token: CancellationTokenWrapper::new(cancellation_token.clone()),
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
            mailbox_cancellation_token: self.mailbox_cancellation_token.clone(),
            cancellation_token: self.cancellation_token.clone(),
        }
    }
}

impl<A: Actor> ActorContext<A> for RuntimeContext<A> {
    type Addr = Addr<A>;
    type CancellationToken = CancellationTokenWrapper;

    fn self_addr(&self) -> &Self::Addr {
        &self.self_addr
    }

    fn stop(&self) {
        self.mailbox_cancellation_token.cancel();
    }

    fn cancellation_token(&self) -> &Self::CancellationToken {
        &self.cancellation_token
    }
}
