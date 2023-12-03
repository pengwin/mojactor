use crate::{Actor, MailboxPreferences};

const MAILBOX_PREFERENCES: MailboxPreferences = MailboxPreferences { size: 1024 };

/// Factory trait for actors
pub trait ActorFactory<A: Actor>: Send + Sync + 'static {
    /// Actor mailbox size
    fn mailbox_preferences(&self) -> &MailboxPreferences {
        &MAILBOX_PREFERENCES
    }
}
