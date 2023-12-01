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
