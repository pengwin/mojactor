use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

/// Atomic timestamp for non precise time measurements
/// Based on `AtomicRefCell`
#[derive(Clone)]
pub struct AtomicTimestamp {
    timestamp: Arc<AtomicInstant>,
}

struct AtomicInstant {
    base: Instant,
    offset: AtomicUsize,
}

impl AtomicInstant {
    fn new(base: Instant) -> AtomicInstant {
        AtomicInstant {
            base,
            offset: AtomicUsize::new(0),
        }
    }

    fn load(&self, order: Ordering) -> Instant {
        let offset_nanos = self.offset.load(order) as u64;
        let secs = offset_nanos / 1_000_000_000;
        let subsec_nanos = (offset_nanos % 1_000_000_000) as u32;
        let offset = Duration::new(secs, subsec_nanos);
        self.base + offset
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_lossless)]
    fn store(&self, val: Instant, order: Ordering) {
        let offset = val - self.base;
        let offset_nanos = offset.as_secs() * 1_000_000_000 + offset.subsec_nanos() as u64;
        self.offset.store(offset_nanos as usize, order);
    }
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
            timestamp: Arc::new(AtomicInstant::new(Instant::now())),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.timestamp.load(Ordering::SeqCst).elapsed()
    }

    pub fn set_now(&self) {
        self.timestamp.store(Instant::now(), Ordering::SeqCst);
    }
}
