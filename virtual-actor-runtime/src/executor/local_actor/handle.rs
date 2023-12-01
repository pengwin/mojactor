//! Implements handle to stop or wait for actor execution

use std::sync::Arc;

use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;
use virtual_actor::Uuid;

use crate::{
    utils::waiter::{waiter, WaitError},
    utils::GracefulShutdown,
};

use super::error::ActorTaskError;

/// Spawned actor handle id
pub type ActorId = Uuid;

/// Spawned task join handle
pub type ActorTaskJoinHandle = tokio::task::JoinHandle<Result<(), ActorTaskError>>;

/// Creates new actor handle id
pub fn generate_actor_id() -> ActorId {
    Uuid::new_v4()
}

/// Local actor handle
pub struct LocalActorHandle {
    /// Spawned actor id
    id: ActorId,
    /// Actor execution task handle
    _task: ActorTaskJoinHandle,
    /// Cancellation token
    cancellation_token: CancellationToken,
    /// Actor stopped notify
    actor_stopped: Arc<Notify>,
}

impl GracefulShutdown for LocalActorHandle {
    async fn graceful_shutdown(&self, timeout: std::time::Duration) -> Result<(), WaitError> {
        self.cancellation_token.cancel();
        waiter(
            "SpawnedActorHandle graceful shutdown",
            &self.actor_stopped,
            timeout,
            None,
        )
        .await
    }
}

impl LocalActorHandle {
    /// Creates new actor handle
    #[must_use]
    pub fn new(
        id: ActorId,
        task: ActorTaskJoinHandle,
        actor_stopped: &Arc<Notify>,
        cancellation_token: &CancellationToken,
    ) -> Self {
        Self {
            id,
            _task: task,
            actor_stopped: actor_stopped.clone(),
            cancellation_token: cancellation_token.clone(),
        }
    }

    /// Gets actor id
    #[must_use]
    pub fn id(&self) -> ActorId {
        self.id
    }
}
