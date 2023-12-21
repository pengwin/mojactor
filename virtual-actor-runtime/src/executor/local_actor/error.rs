//! Error produced by actor task

use virtual_actor::ResponderError;

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
}

#[derive(Debug, thiserror::Error)]
pub enum ActorSpawnError {
    /// Dispatcher already set
    #[error("Dispatcher already set {0}")]
    DispatcherAlreadySet(&'static str),
    /// Actor task already set
    #[error("Actor task already set {0}")]
    ActorTaskAlreadySet(&'static str),
}
