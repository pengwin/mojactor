//! Context factory trait

use std::sync::Arc;

use virtual_actor::{Actor, ActorContext};

use crate::address::{ActorHandle, Addr};

/// Context factory
pub trait ActorContextFactory<A>: Send + Sync + 'static
where
    A: Actor,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
{
    /// Creates new context
    fn create_context(&self, handle: &Arc<ActorHandle<A>>) -> A::ActorContext;
}
