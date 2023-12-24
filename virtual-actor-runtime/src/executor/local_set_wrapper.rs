use futures::FutureExt;
use std::future::Future;
use tokio::{
    runtime::Runtime,
    select,
    sync::Notify,
    task::{JoinHandle, LocalSet},
};
use tokio_util::sync::CancellationToken;

/// Wraps `LocalSet`
/// Provides graceful shutdown implementation
pub struct LocalSetWrapper {
    local: LocalSet,
}

impl LocalSetWrapper {
    /// Creates new `LocalSetWrapper`
    pub fn new() -> Self {
        Self {
            local: LocalSet::new(),
        }
    }

    /// Spawns new task
    pub fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        self.local.spawn_local(future)
    }

    /// Runs local set
    pub fn run(self, rt: &Runtime, notify: &Notify, cancellation_token: &CancellationToken) {
        rt.block_on(self.inner_run_with_cancellation(notify, cancellation_token));
    }

    async fn inner_run_with_cancellation(
        self,
        notify: &Notify,
        cancellation_token: &CancellationToken,
    ) {
        select! {
            biased;
            () = cancellation_token.cancelled() => {
                eprintln!("Local set cancelled");
            },
            () = self.local.inspect(move |()| { notify.notify_one(); }) => {},
        }
    }
}
