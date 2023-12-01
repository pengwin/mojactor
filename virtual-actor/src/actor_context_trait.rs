//! Trait for message processing context

use crate::{actor_addr::ActorAddr, Actor};

/// Context for message processing
pub trait ActorContext<A: Actor>: Clone {
    /// Actor address type
    type Addr: ActorAddr<A>;

    /// Returns actor reference
    fn self_addr(&self) -> &Self::Addr;

    /// Send stop signal to actor
    fn stop(&self);
}
