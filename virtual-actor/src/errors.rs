//! Module contains error types

/// Message processing error
#[derive(Debug, thiserror::Error)]
pub enum MessageProcessingError {
    /// Message handling panic
    #[error("Message processing panic {0:?}")]
    Panic(String),
}

/// Responder error
#[derive(Debug, thiserror::Error)]
pub enum ResponderError {
    /// Communication channel error
    #[error("ResponderChannelError {0:?}")]
    ChannelError(&'static str),
    /// Response was already used
    #[error("AlreadyRespond {0}")]
    AlreadyRespond(&'static str),
}
