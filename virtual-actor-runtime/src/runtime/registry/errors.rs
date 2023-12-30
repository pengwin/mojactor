use virtual_actor::actor::ActorName;

use crate::{
    address::errors::{ActorStartError, LocalAddrError},
    errors::WaitError,
    executor::errors::LocalExecutorError,
};

/// Housekeeping actor start error
#[derive(Debug, thiserror::Error)]
pub enum StartHousekeepingError {
    /// Wait dispatcher error
    #[error("WaitDispatcherError {0:?}")]
    WaitDispatcher(#[from] WaitError),
    /// Send message error
    #[error("StartGarbageCollectError {0:?}")]
    StartGarbageCollect(#[from] LocalAddrError),
    /// Actor start error
    #[error("Actor start error {0:?}")]
    ActorStart(#[from] ActorStartError),
}

/// Actor spawn error
#[derive(Debug, thiserror::Error)]
pub enum RuntimeSpawnError {
    /// Start housekeeping error
    #[error("StartHousekeepingError {0:?}")]
    StartHousekeeping(#[from] StartHousekeepingError),
    /// Wait dispatcher error
    #[error("WaitDispatcherError {0:?}")]
    WaitDispatcher(#[from] WaitError),
    /// Executor error
    #[error("ExecutorError {0:?}")]
    ExecutorError(#[from] LocalExecutorError),
    /// Actor start error
    #[error("Actor start error {0:?}")]
    ActorStartError(#[from] ActorStartError),
}

/// Actor activation error
#[derive(Debug, thiserror::Error)]
pub enum ActivateActorError {
    /// Actor spawn error
    #[error("Actor {0:?} not found")]
    ActorNotFound(ActorName),
    #[error("Actor registry dropped")]
    /// Actor registry dropped
    ActorRegistryDropped,
    #[error("Unexpected activator registered for actor {0:?}")]
    /// Unexpected activator registered for actor
    UnexpectedActivator(ActorName),
    /// Actor spawn error
    #[error("ActorSpawnError: {0:?}")]
    SpawnError(#[from] RuntimeSpawnError),
}
