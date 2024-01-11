//! Error produced by actor task

use tokio::task::JoinError;
use virtual_actor::errors::{BoxedActorError, ResponderError};

use crate::address::errors::ActorTaskContainerError;

/// Error produced by actor task
#[derive(Debug, thiserror::Error)]
pub enum ActorTaskError {
    /// Actor task panic
    #[error("Actor task panic {0}")]
    ActorPanic(String),
    /// Responder error
    #[error("Responder error {0:?}")]
    ResponderError(#[from] ResponderError),
    /// Cancelled
    #[error("Cancelled")]
    Cancelled,
    /// Actor factory error
    #[error("Actor factory error {0:?}")]
    ActorFactoryError(BoxedActorError),
    /// Task join error
    #[error("Task join error {0:?}")]
    TaskJoinError(#[from] JoinError),
    /// Before message hook error
    #[error("Before message hook error {0:?}")]
    BeforeMessageHookError(BoxedActorError),
    /// After message hook error
    #[error("After message hook error {0:?}")]
    AfterMessageHookError(BoxedActorError),
}

impl ActorTaskError {
    /// Creates new actor factory error
    #[must_use]
    pub fn actor_factory_error<E: std::error::Error + 'static + Send + Sync>(e: E) -> Self {
        Self::ActorFactoryError(BoxedActorError::new(e))
    }
}

/// Error occurred during actor spawn
#[derive(Debug, thiserror::Error)]
pub enum ActorSpawnError {
    /// Dispatcher already set
    #[error("Dispatcher already set {0}")]
    DispatcherAlreadySet(&'static str),
    /// Actor task already set
    #[error("Actor task already set {0:?}")]
    ActorTaskContainerError(#[from] ActorTaskContainerError),
}
