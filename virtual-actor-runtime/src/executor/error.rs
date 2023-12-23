//! Error for `LocalExecutor`

/// `LocalExecutor` errors
#[derive(thiserror::Error, Debug)]
pub enum LocalExecutorError {
    /// Executor thread already started
    #[error("Executor thread already started")]
    ExecutorThreadAlreadyStarted,
    /// Executor thread already started
    #[error("Executor thread not started")]
    ExecutorThreadNotStarted,
    /// Tokio runtime error
    #[error("Runtime error {0}")]
    RuntimeError(#[from] tokio::io::Error),
    /// Thread receive error
    #[error("Thread receive error {0}")]
    ThreadReceiveError(#[from] std::sync::mpsc::RecvError),
    /// Spawner send error
    #[error("Spawner send error {0}")]
    SpawnerSendError(String),
    /// Dispatcher wait error
    #[error("Dispatcher wait error {0}")]
    DispatcherWaitError(#[from] crate::utils::waiter::WaitError),
    /// Unable to start thread
    #[error("Unable to spawn thread {0:?}")]
    ThreadSpawnError(std::io::Error),
}
