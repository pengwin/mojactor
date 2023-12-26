use core::fmt;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use virtual_actor::VirtualActor;

use crate::address::ActorHandle;

pub struct CountersInfo {
    dispatched: usize,
    processed: usize,
    timestamp: Instant,
}

impl fmt::Debug for CountersInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CountersInfo")
            .field("dispatched", &self.dispatched)
            .field("processed", &self.processed)
            .field("timestamp", &self.timestamp.elapsed())
            .finish()
    }
}

pub struct ActorCountersMap<A: VirtualActor> {
    map: HashMap<A::ActorId, CountersInfo>,
}

impl<A: VirtualActor> ActorCountersMap<A> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Updates counters and timestamp for actor with given id.
    /// If actor with given id is not present in map, it will be added.
    /// If actor with given id is present in map, but counters are the same, nothing will be changed.
    pub fn update(&mut self, actor_id: &A::ActorId, handle: &ActorHandle<A>) {
        let dispatched = handle.dispatched_msg_counter().get();
        let processed = handle.processed_msg_counter().get();

        self.map
            .entry(actor_id.clone())
            .and_modify(|counters_info| {
                if counters_info.dispatched == dispatched && counters_info.processed == processed {
                    return;
                }
                counters_info.dispatched = dispatched;
                counters_info.processed = processed;
                counters_info.timestamp = Instant::now();
            })
            .or_insert(CountersInfo {
                dispatched,
                processed,
                timestamp: Instant::now(),
            });
    }

    /// Removes actor with given id from map.
    pub fn remove(&mut self, actor_id: &A::ActorId) {
        self.map.remove(actor_id);
    }

    /// Returns true if actor with given id is idle for given `idle_time`.
    pub fn is_idle(&self, id: &A::ActorId, idle_time: Duration) -> bool {
        if let Some(counters_info) = self.map.get(id) {
            counters_info.dispatched == counters_info.processed
                && counters_info.timestamp.elapsed() >= idle_time
        } else {
            false
        }
    }
}
