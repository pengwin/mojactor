use futures::future::BoxFuture;
use virtual_actor_runtime::errors::BoxedActorError;

use crate::actor_with_state_trait::ActorWithState;

/// Actor persistence
pub trait ActorPersistence<A: ActorWithState>: Send + Sync + 'static {
    /// Load state
    fn load(&self, id: &A::ActorId) -> BoxFuture<Result<Option<A::State>, BoxedActorError>>;

    /// Save state
    fn save(&self, id: &A::ActorId, state: &A::State) -> BoxFuture<Result<(), BoxedActorError>>;

    /// Clear state
    fn clear(&self, id: &A::ActorId) -> BoxFuture<Result<(), BoxedActorError>>;
}
