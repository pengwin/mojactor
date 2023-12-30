use super::{Actor, ActorAddr};

/// Weak actor address
pub trait WeakActorAddr<A: Actor>: Send + Sync + Clone {
    /// Upgraded actor address
    type Upgraded: ActorAddr<A, WeakRef = Self>;

    /// Attempts to upgrade `WeakActorRef` to `ActorAddr`
    fn upgrade(&self) -> Option<Self::Upgraded>;
}