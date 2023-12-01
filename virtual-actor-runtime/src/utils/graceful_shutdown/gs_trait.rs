//! Graceful shutdown trait

use std::future::Future;

use crate::utils::waiter::WaitError;

/// Graceful shutdown trait
pub trait GracefulShutdown {
    /// Graceful shutdown
    ///
    /// # Errors
    ///
    /// Returns `WaitError::Timeout` if graceful shutdown timeout exceeded
    /// Returns `WaitError::Cancelled` if graceful shutdown cancelled
    fn graceful_shutdown(
        &self,
        timeout: std::time::Duration,
    ) -> impl Future<Output = Result<(), WaitError>>;
}
