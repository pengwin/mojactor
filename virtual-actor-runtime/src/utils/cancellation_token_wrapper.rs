use tokio_util::sync::CancellationToken;

use virtual_actor::CancellationToken as CancellationTokenTrait;

/// Cancellation token wrapper around `tokio_util::sync::CancellationToken`
#[derive(Clone)]
pub struct CancellationTokenWrapper {
    token: CancellationToken,
}

impl CancellationTokenWrapper {
    /// Wraps `tokio_util::sync::CancellationToken`
    pub fn new(token: CancellationToken) -> Self {
        Self { token }
    }
}

impl CancellationTokenTrait for CancellationTokenWrapper {
    async fn cancelled(&self) {
        self.token.cancelled().await;
    }
}
