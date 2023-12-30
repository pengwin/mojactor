use virtual_actor::errors::MessageProcessingError;

use crate::{
    errors::WaitError, executor::errors::ActorTaskError, messaging::errors::DispatcherError,
    runtime::errors::RuntimeSpawnError,
};

/// Error returned by container for actor task
#[derive(thiserror::Error, Debug)]
#[error("Actor task container error: {message}")]
pub struct ActorTaskContainerError {
    /// Error message
    pub message: String,
}

/// Actor start error
#[derive(thiserror::Error, Debug)]
pub enum ActorStartError {
    /// Start wait error
    #[error("Start wait error: {0:?}")]
    WaitError(#[from] WaitError),
    /// Actor task error
    #[error("ActorTaskError: {0:?}")]
    ActorTaskError(#[from] ActorTaskError),
    /// Unexpected state
    #[error("Unexpected state {0}")]
    UnexpectedState(String),
}

/// Actor handler error
#[derive(thiserror::Error, Debug)]
pub enum LocalAddrError {
    /// Dispatcher not set
    #[error("Actor not ready to receive messages")]
    ActorNotReady,
    /// Dispatcher not set
    #[error("Actor stopped")]
    Stopped,
    /// Dispatcher error
    #[error("Dispatch error {0:?}")]
    DispatcherError(#[from] DispatcherError),
    /// Message processing error
    #[error("Message processing error {0:?}")]
    MessageProcessingError(#[from] MessageProcessingError),
}

/// Actor handler error
#[derive(thiserror::Error, Debug)]
pub enum VirtualAddrError {
    /// Actor spawn error
    #[error("RuntimeSpawnError {0:?}")]
    SpawnError(#[from] RuntimeSpawnError),
    /// Dispatcher error
    #[error("LocalAddrError {0:?}")]
    LocalAddrError(#[from] LocalAddrError),
    /// Actor task error
    #[error("ActorTaskContainerError {0:?}")]
    ActorTaskContainerError(#[from] ActorTaskContainerError),
}
