//! Trait for message processing context

use crate::{actor_addr::ActorAddr, Actor, CancellationToken};

/// Context for message processing
pub trait ActorContext<A: Actor>: Clone {
    /// Actor address type
    type Addr: ActorAddr<A>;
    /// Cancellation token
    type CancellationToken: CancellationToken;

    /// Returns actor reference
    fn self_addr(&self) -> &<Self::Addr as ActorAddr<A>>::WeakRef;

    /// Send stop signal to actor
    fn stop(&self);

    /// Cancellation token
    fn cancellation_token(&self) -> &Self::CancellationToken;
}
