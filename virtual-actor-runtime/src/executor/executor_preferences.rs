use virtual_actor::MailboxPreferences;

/// Local executor preferences
pub struct ExecutorPreferences {
    /// Mailbox preferences
    pub mailbox_preferences: MailboxPreferences,
}

impl Default for ExecutorPreferences {
    fn default() -> Self {
        Self {
            mailbox_preferences: MailboxPreferences { size: 1024 },
        }
    }
}
