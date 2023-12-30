//! Local spawner implementation

use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;
use virtual_actor::message::MailboxPreferences;

use crate::utils::GracefulShutdownHandle;

use super::{mailbox::Mailbox, SpawnerDispatcher};

/// Local spawner.
/// Spawns actors on `LocalSet`
pub struct LocalSpawner {
    mailbox: Mailbox,
    dispatcher: SpawnerDispatcher,
    cancellation_token: CancellationToken,
    /// Notify when spawn loop is stopped
    stopped_notify: Arc<Notify>,
}

impl LocalSpawner {
    /// Creates new `LocalSpawner`
    pub fn new(
        mailbox_preferences: &MailboxPreferences,
        mailbox_cancellation: &CancellationToken,
        cancellation_token: &CancellationToken,
    ) -> Self {
        let (dispatcher, mailbox) = Mailbox::new(mailbox_preferences, mailbox_cancellation);
        let stopped_notify = Arc::new(Notify::new());
        Self {
            mailbox,
            dispatcher,
            cancellation_token: cancellation_token.clone(),
            stopped_notify,
        }
    }

    /// Gets dispatcher clone
    pub fn dispatcher(&self) -> &SpawnerDispatcher {
        &self.dispatcher
    }

    pub fn graceful_shutdown_handle(&self) -> GracefulShutdownHandle {
        GracefulShutdownHandle::new(
            stringify!(LocalSpawner),
            self.stopped_notify.clone(),
            self.cancellation_token.clone(),
        )
    }

    /// Start spawn loop
    /// Spawn actors from mailbox
    pub async fn run(mut self) {
        let notify = self.stopped_notify.clone();
        self.inner_loop()
            .inspect(move |()| notify.notify_one())
            .await;
    }

    async fn inner_loop(&mut self) {
        while let Some(new_actor) = self.mailbox.recv(&self.cancellation_token).await {
            if let Err(e) = new_actor.spawn() {
                eprintln!("Failed to spawn actor: {e:?}");
            }
        }
    }
}
