use crate::Message;

/// Message processing error
#[derive(Debug, thiserror::Error)]
pub enum MessageProcessingError {
    /// Message handling panic
    #[error("Message processing panic {0:?}")]
    Panic(String),
}

/// Message processing result
pub type MessageProcessingResult<M> = Result<<M as Message>::Result, MessageProcessingError>;
