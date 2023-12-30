use crate::VirtualActor;

/// Constructor trait for virtual actors
pub trait VirtualActorConstructor: VirtualActor {
    /// Creates new virtual actor
    #[must_use]
    fn new(id: &Self::ActorId) -> Self;
}
