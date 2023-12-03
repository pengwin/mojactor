use std::future::Future;

use crate::{ActorFactory, VirtualActor};

/// Factory trait for virtual actor
pub trait VirtualActorFactory<A: VirtualActor>: ActorFactory<A> {
    /// Creates new virtual actor
    fn create_actor(&self, id: &A::ActorId) -> impl Future<Output = A>;
}
