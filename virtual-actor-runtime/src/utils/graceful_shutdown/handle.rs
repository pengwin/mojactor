use std::sync::Arc;

use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::{
    utils::waiter::{waiter, WaitError},
    GracefulShutdown,
};

/// Graceful shutdown handle
pub struct GracefulShutdownHandle {
    name: String,
    cancellation_token: CancellationToken,
    /// Notify when spawn loop is stopped
    notify: Arc<Notify>,
}

impl GracefulShutdownHandle {
    /// Creates new `GracefulShutdownHandle`
    pub fn new(name: &str, notify: Arc<Notify>, cancellation_token: CancellationToken) -> Self {
        Self {
            name: name.to_owned(),
            cancellation_token,
            notify,
        }
    }

    /// Wait for finish without cancellation
    pub async fn wait(&self, timeout: std::time::Duration) -> Result<(), WaitError> {
        waiter(&self.name, &self.notify, timeout, None).await
    }

    /// Shutdown, without wait
    pub fn shutdown(&self) {
        self.cancellation_token.cancel();
    }
}

impl GracefulShutdown for GracefulShutdownHandle {
    async fn graceful_shutdown(self, timeout: std::time::Duration) -> Result<(), WaitError> {
        self.shutdown();
        self.wait(timeout).await
    }
}
