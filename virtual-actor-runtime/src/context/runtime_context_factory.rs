use std::{marker::PhantomData, sync::Arc};

use virtual_actor::{Actor, ActorContext};

use crate::{address::ActorHandle, Addr};

use super::{context_factory_trait::ActorContextFactory, runtime_context::RuntimeContext};

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
    fn create_context(&self, handle: &Arc<ActorHandle<A>>) -> A::ActorContext {
        let addr = Addr::new(handle);
        RuntimeContext::new(
            addr,
            handle.mailbox_cancellation(),
            handle.cancellation_token(),
        )
    }
}
