//! Runtime context for actor.

use std::marker::PhantomData;

use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, ActorContext};

use crate::{addr::Addr, context_factory_trait::ActorContextFactory};

/// Runtime context for actor.
pub struct RuntimeContext<A: Actor> {
    /// Actor address
    self_addr: Addr<A>,
    /// Cancellation token
    cancellation_token: CancellationToken,
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

/// Runtime context factory
pub struct RuntimeContextFactory<A: Actor> {
    /// Phantom data
    _a: PhantomData<fn(A) -> A>,
}

impl<A: Actor> Default for RuntimeContextFactory<A> {
    fn default() -> Self {
        Self { _a: PhantomData }
    }
}

impl<A> ActorContextFactory<A> for RuntimeContextFactory<A>
where
    A: Actor<ActorContext = RuntimeContext<A>> + 'static,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
{
    fn create_context(
        &self,
        addr: Addr<A>,
        cancellation_token: &CancellationToken,
    ) -> A::ActorContext {
        RuntimeContext {
            self_addr: addr,
            cancellation_token: cancellation_token.clone(),
        }
    }
}
