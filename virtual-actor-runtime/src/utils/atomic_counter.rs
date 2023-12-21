use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Clone)]
pub struct AtomicCounter {
    counter: Arc<AtomicUsize>,
}

impl AtomicCounter {
    pub fn get(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    pub fn increment(&self) -> usize {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self {
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }
}
