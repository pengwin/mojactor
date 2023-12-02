//! Mailbox for local spawner

use tokio::{
    select,
    sync::mpsc::{Receiver, Sender},
};
use tokio_util::sync::CancellationToken;
use virtual_actor::MailboxPreferences;

/// Dispatcher for mailbox
pub type MailboxDispatcher<T> = Sender<T>;

/// Base mailbox implementation
pub struct Mailbox<T> {
    /// Mailbox channel
    receiver: Receiver<T>,
    /// Message receiving cancellation token
    receiver_cancellation: CancellationToken,
    /// is closed
    closed: bool,
}

impl<T> Mailbox<T> {
    /// Creates new mailbox
    pub fn new(
        preferences: &MailboxPreferences,
        mailbox_cancellation: &CancellationToken,
    ) -> (MailboxDispatcher<T>, Self) {
        let (dispatcher, mailbox) = tokio::sync::mpsc::channel::<T>(preferences.size);
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
    pub async fn recv(&mut self, ct: &CancellationToken) -> Option<T> {
        // close channel if cancellation token is cancelled
        // caller will read the rest of messages from channel and then receive None
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

    /// Receive messages with checking mailbox cancellation token
    /// it will close channel if cancellation token is cancelled
    async fn recv_with_mailbox_ct(&mut self, ct: &CancellationToken) -> Option<T> {
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

    /// Receive message with cancellation token
    /// to interrupt waiting for message
    async fn recv_with_ct(&mut self, ct: &CancellationToken) -> Option<T> {
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
    use virtual_actor::MailboxPreferences;

    #[tokio::test]
    async fn test_mailbox() {
        let mailbox_ct = CancellationToken::new();
        let (dispatcher, mut mailbox) =
            super::Mailbox::new(&MailboxPreferences { size: 10 }, &mailbox_ct);

        dispatcher.try_send(123).expect("Send message to mailbox");
        dispatcher.try_send(123).expect("Send message to mailbox");
        dispatcher.try_send(123).expect("Send message to mailbox");

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
}
