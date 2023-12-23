use virtual_actor::MailboxPreferences;

/// Preferences for the Tokio runtime fro executor
pub struct TokioRuntimePreferences {
    /// Enable I/O
    pub enable_io: bool,
    /// Enable time
    pub enable_time: bool,
    /// The stack size (in bytes) for executor threads.
    pub thread_stack_size: Option<usize>,
}

impl Default for TokioRuntimePreferences {
    fn default() -> Self {
        Self {
            enable_io: true,
            enable_time: true,
            thread_stack_size: None,
        }
    }
}

/// Local executor preferences
pub struct ExecutorPreferences {
    /// Executor thread name
    pub thread_name: String,
    /// Mailbox preferences
    pub mailbox_preferences: MailboxPreferences,
    /// Tokio runtime preferences
    pub tokio_runtime_preferences: TokioRuntimePreferences,
}

impl Default for ExecutorPreferences {
    fn default() -> Self {
        Self {
            thread_name: "local-executor".to_string(),
            mailbox_preferences: MailboxPreferences { size: 1024 },
            tokio_runtime_preferences: TokioRuntimePreferences::default(),
        }
    }
}
