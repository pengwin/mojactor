use std::time::Duration;

pub struct RuntimePreferences {
    pub garbage_collect_interval: Duration,
    pub actor_idle_timeout: Duration,
    pub actor_activation_timeout: Duration,
    pub actor_shutdown_interval: Duration,
}

impl Default for RuntimePreferences {
    fn default() -> Self {
        Self {
            garbage_collect_interval: Duration::from_secs(10),
            actor_idle_timeout: Duration::from_secs(1),
            actor_activation_timeout: Duration::from_secs(1),
            actor_shutdown_interval: Duration::from_secs(1),
        }
    }
}
