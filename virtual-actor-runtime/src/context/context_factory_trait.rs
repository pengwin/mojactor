//! Context factory trait

use virtual_actor::{Actor, ActorContext};

use crate::address::{ActorHandle, LocalAddr};

/// Context factory
pub trait ActorContextFactory<A>: Send + Sync + 'static
where
    A: Actor,
    A::ActorContext: ActorContext<A, Addr = LocalAddr<A>>,
{
    /// Creates new context
    fn create_context(&self, handle: &ActorHandle<A>) -> A::ActorContext;
}
