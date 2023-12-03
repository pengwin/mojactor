//! Mailbox for local spawner

use tokio_util::sync::CancellationToken;
use virtual_actor::MailboxPreferences;

use super::{super::local_actor::LocalSpawnedActor, SpawnerDispatcher};
use crate::messaging::Mailbox as BaseMailbox;

/// Mailbox for `LocalSpawner`
pub struct Mailbox {
    inner: BaseMailbox<Box<dyn LocalSpawnedActor>>,
}

impl Mailbox {
    /// Creates new mailbox
    pub fn new(
        preferences: &MailboxPreferences,
        mailbox_cancellation: &CancellationToken,
    ) -> (SpawnerDispatcher, Self) {
        let (sender, inner) = BaseMailbox::new(preferences, mailbox_cancellation);
        let dispatcher = SpawnerDispatcher::new(sender);
        (dispatcher, Self { inner })
    }

    /// Receive message from mailbox
    pub async fn recv(&mut self, ct: &CancellationToken) -> Option<Box<dyn LocalSpawnedActor>> {
        self.inner.recv(ct).await
    }
}

#[cfg(test)]
mod tests {
    use tokio::{
        select,
        time::{sleep, Duration},
    };
    use tokio_util::sync::CancellationToken;
    use virtual_actor::MailboxPreferences;

    use crate::executor::actor_registry::ActorRegistry;

    #[tokio::test]
    async fn test_mailbox() {
        let mailbox_ct = CancellationToken::new();
        let (dispatcher, mut mailbox) =
            super::Mailbox::new(&MailboxPreferences { size: 10 }, &mailbox_ct);

        dispatcher
            .send(Box::new(TestSpawner))
            .expect("Send message to mailbox");
        dispatcher
            .send(Box::new(TestSpawner))
            .expect("Send message to mailbox");
        dispatcher
            .send(Box::new(TestSpawner))
            .expect("Send message to mailbox");

        mailbox_ct.cancel();

        let e = mailbox.recv(&CancellationToken::new()).await;
        assert!(e.is_some(), "Mailbox should return messages");

        assert!(
            dispatcher.is_closed(),
            "Mailbox dispatcher should be closed"
        );

        let drain_ct = CancellationToken::new();

        let drain_mailbox = async move {
            let mut counter = 0;
            while select! {
                () = sleep(Duration::from_millis(5)) => panic!("Drain timeout"),
                e = mailbox.recv(&drain_ct) => e,
            }
            .is_some()
            {
                counter += 1;
                continue;
            }

            assert_eq!(counter, 2, "Mailbox should be drained");
        };

        drain_mailbox.await;
    }

    struct TestSpawner;

    impl super::LocalSpawnedActor for TestSpawner {
        fn spawn(&self, _: &ActorRegistry) {
            todo!()
        }
    }
}
