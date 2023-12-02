//! Context factory trait

use tokio_util::sync::CancellationToken;
use virtual_actor::{Actor, ActorContext};

use crate::address::Addr;

/// Context factory
pub trait ActorContextFactory<A>: Send + Sync + 'static
where
    A: Actor,
    A::ActorContext: ActorContext<A, Addr = Addr<A>>,
{
    /// Creates new context
    fn create_context(
        &self,
        addr: <A::ActorContext as ActorContext<A>>::Addr,
        cancellation_token: &CancellationToken,
    ) -> A::ActorContext;
}
