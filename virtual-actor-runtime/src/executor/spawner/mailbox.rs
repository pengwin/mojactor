//! Mailbox for local spawner

use tokio::{
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

use super::super::local_actor::LocalActor;

/// Dispatcher for `LocalSpawner`
pub type SpawnerDispatcher = UnboundedSender<Box<dyn LocalActor>>;

/// Mailbox for `LocalSpawner`
pub struct Mailbox {
    /// Mailbox channel
    receiver: UnboundedReceiver<Box<dyn LocalActor>>,
    /// Message receiving cancellation token
    receiver_cancellation: CancellationToken,
    /// is closed
    closed: bool,
}

impl Mailbox {
    /// Creates new mailbox
    pub fn new(mailbox_cancellation: &CancellationToken) -> (SpawnerDispatcher, Self) {
        let (dispatcher, mailbox) = tokio::sync::mpsc::unbounded_channel::<Box<dyn LocalActor>>();
        (
            dispatcher,
            Self {
                closed: false,
                receiver: mailbox,
                receiver_cancellation: mailbox_cancellation.clone(),
            },
        )
    }

    /// Receive message from mailbox
    pub async fn recv(&mut self, ct: &CancellationToken) -> Option<Box<dyn LocalActor>> {
        if !self.closed && self.receiver_cancellation.is_cancelled() {
            self.receiver.close();
            self.closed = true;
        }

        if self.closed {
            self.recv_with_ct(ct).await
        } else {
            self.recv_with_mailbox_ct(ct).await
        }
    }

    async fn recv_with_mailbox_ct(
        &mut self,
        ct: &CancellationToken,
    ) -> Option<Box<dyn LocalActor>> {
        let mailbox_ct = self.receiver_cancellation.clone();
        select! {
            () = mailbox_ct.cancelled() => {
                self.receiver.close();
                self.closed = true;
                self.recv_with_ct(&ct.clone()).await
            },
            envelope = self.recv_with_ct(ct) => envelope,
        }
    }

    async fn recv_with_ct(&mut self, ct: &CancellationToken) -> Option<Box<dyn LocalActor>> {
        select! {
            () = ct.cancelled() => None,
            envelope = self.receiver.recv() => envelope,
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::{
        select,
        time::{sleep, Duration},
    };
    use tokio_util::sync::CancellationToken;

    use crate::executor::actor_registry::ActorRegistry;

    #[tokio::test]
    async fn test_mailbox() {
        let mailbox_ct = CancellationToken::new();
        let (dispatcher, mut mailbox) = super::Mailbox::new(&mailbox_ct);

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

    impl super::LocalActor for TestSpawner {
        fn spawn(&self, _: &ActorRegistry) {
            todo!()
        }
    }
}
