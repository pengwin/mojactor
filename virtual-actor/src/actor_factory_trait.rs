use std::future::Future;

use crate::{Actor, MailboxPreferences};

const MAILBOX_PREFERENCES: MailboxPreferences = MailboxPreferences { size: 1024 };

/// Factory trait for actors
pub trait ActorFactory<A: Actor>: Send + Sync + 'static {
    /// Creates new actor
    fn create_actor(&self) -> impl Future<Output = A>;

    /// Actor mailbox size
    fn mailbox_preferences(&self) -> &MailboxPreferences {
        &MAILBOX_PREFERENCES
    }
}
