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
    /// Actor factory error
    #[error("Actor factory error {0:?}")]
    ActorFactoryError(Box<dyn std::error::Error + 'static + Send + Sync>),
}

impl ActorTaskError {
    /// Creates new actor factory error
    #[must_use]
    pub fn actor_factory_error<E: std::error::Error + 'static + Send + Sync>(e: E) -> Self {
        Self::ActorFactoryError(Box::new(e))
    }
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
