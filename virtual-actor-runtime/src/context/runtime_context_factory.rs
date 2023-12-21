use std::{marker::PhantomData, sync::Arc};

use virtual_actor::{Actor, ActorContext};

use crate::{address::ActorHandle, runtime::ActorRegistry, LocalAddr, WeakLocalAddr};

use super::{context_factory_trait::ActorContextFactory, runtime_context::RuntimeContext};

/// Runtime context factory
pub struct RuntimeContextFactory<A: Actor> {
    /// Phantom data
    _a: PhantomData<fn(A) -> A>,
    /// Actor registry
    registry: Arc<ActorRegistry>,
}

impl<A: Actor> RuntimeContextFactory<A> {
    /// Creates new runtime context factory
    #[must_use]
    pub fn new(registry: Arc<ActorRegistry>) -> Self {
        Self {
            _a: PhantomData,
            registry,
        }
    }
}

impl<A> ActorContextFactory<A> for RuntimeContextFactory<A>
where
    A: Actor<ActorContext = RuntimeContext<A>> + 'static,
    A::ActorContext: ActorContext<A, Addr = LocalAddr<A>>,
{
    fn create_context(&self, handle: &ActorHandle<A>) -> A::ActorContext {
        let weak_addr = WeakLocalAddr::new(handle);
        RuntimeContext::new(
            self.registry.clone(),
            weak_addr,
            handle.mailbox_cancellation(),
            handle.cancellation_token(),
        )
    }
}
