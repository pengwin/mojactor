use super::LocalActor;

/// Constructor trait for local actors
pub trait LocalActorConstructor: LocalActor {
    /// Creates new local actor
    #[must_use]
    fn new() -> Self;
}

/// Blanket implementation of `LocalActorConstructor` for all types that implement `LocalActor` and `Default`
impl<A: LocalActor + Default> LocalActorConstructor for A {
    fn new() -> Self {
        Self::default()
    }
}
