use std::time::{Duration, Instant};

use atomic_refcell::AtomicRefCell;

/// Atomic timestamp for non precise time measurements
/// Based on `AtomicRefCell`
#[derive(Clone)]
pub struct AtomicTimestamp {
    timestamp: AtomicRefCell<Instant>,
}

impl Default for AtomicTimestamp {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomicTimestamp {
    /// Creates new atomic timestamp
    #[must_use]
    pub fn new() -> AtomicTimestamp {
        Self {
            timestamp: AtomicRefCell::new(Instant::now()),
        }
    }

    pub fn elapsed(&self) -> Duration {
        let timestamp = self.timestamp.borrow();
        let res = timestamp.elapsed();
        drop(timestamp);
        res
    }

    pub fn set_now(&self) {
        let mut timestamp = self.timestamp.borrow_mut();
        *timestamp = Instant::now();
        drop(timestamp);
    }
}
