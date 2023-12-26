use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::sync::Notify;

pub struct NotifyOnce {
    inner: Arc<Notify>,
    finished: Arc<AtomicBool>,
}

impl NotifyOnce {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Notify::new()),
            finished: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn notify(&self) {
        if self.is_notified() {
            return;
        }
        self.inner.notify_one();
        self.finished.store(true, Ordering::SeqCst);
    }

    pub async fn wait_for_notify(&self) {
        self.inner.notified().await;
    }

    pub fn inner(&self) -> &Notify {
        &self.inner
    }

    pub fn is_notified(&self) -> bool {
        self.finished.load(Ordering::SeqCst)
    }
}

impl Default for NotifyOnce {
    fn default() -> Self {
        Self::new()
    }
}
