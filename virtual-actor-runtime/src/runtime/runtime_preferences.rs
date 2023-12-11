use std::time::Duration;

pub struct RuntimePreferences {
    pub garbage_collect_interval: Duration,
}

impl Default for RuntimePreferences {
    fn default() -> Self {
        Self {
            garbage_collect_interval: Duration::from_secs(10),
        }
    }
}
