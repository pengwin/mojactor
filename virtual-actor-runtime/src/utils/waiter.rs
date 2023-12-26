//! Utility functions for waiter

use std::time::Duration;

use tokio::{select, sync::Notify};
use tokio_util::sync::CancellationToken;

/// Error type for waiting processes
#[derive(Debug, thiserror::Error)]
pub enum WaitError {
    /// Wait timeout
    #[error("Wait timeout: {0:?}")]
    Timeout(String),
    /// Wait cancelled
    #[error("Wait cancelled: {0:?}")]
    Cancelled(String),
}

/// Wait for notification
pub async fn waiter(
    name: &str,
    notify: &Notify,
    timeout: Duration,
    cancellation_token: Option<&CancellationToken>,
) -> Result<(), WaitError> {
    let cancellation = async {
        match cancellation_token {
            Some(token) => token.cancelled().await,
            None => futures::future::pending().await,
        }
    };
    select! {
        biased;
        () = cancellation => Err(WaitError::Cancelled(name.to_owned())),
        () = notify.notified() => Ok(()),
        () = tokio::time::sleep(timeout) => Err(WaitError::Timeout(name.to_owned())),
    }
}
