use std::time::Duration;

/// Runtime settings
pub struct RuntimePreferences {
    /// Interval for actors garbage collection
    pub garbage_collect_interval: Duration,
    /// Timeout for actor to be idle before it will be collected
    pub actor_idle_timeout: Duration,
    /// Timeout for actor to wait for activation before returning an error
    pub actor_activation_timeout: Duration,
    /// Timeout for actor to wait for shutdown before during graceful shutdown
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
