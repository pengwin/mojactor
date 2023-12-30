use tokio::sync::{mpsc::error::TrySendError, oneshot::error::RecvError};

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

/// Message dispatcher error
#[derive(thiserror::Error, Debug)]
pub enum DispatcherError {
    /// Mailbox send error
    #[error("Mailbox error: {0:?}")]
    MailBoxError(#[from] MailboxError),
    /// Response receiver error
    #[error("Response receiver error: {0:?}")]
    ResponseReceiverError(#[from] RecvError),
}

impl DispatcherError {
    pub fn from_try_send_error<T>(e: TrySendError<T>) -> Self {
        Self::MailBoxError(MailboxError::from_try_send_error(e))
    }
}
