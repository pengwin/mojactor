use tokio::sync::mpsc::error::TrySendError;

#[derive(thiserror::Error, Debug)]
pub enum MailboxError {
    #[error("Mailbox is closed")]
    Closed,
    #[error("Mailbox is full")]
    Full,
}

impl MailboxError {
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_try_send_error<T>(e: TrySendError<T>) -> Self {
        match e {
            TrySendError::Closed(_) => Self::Closed,
            TrySendError::Full(_) => Self::Full,
        }
    }
}
