//! Responder trait for responders actor messages

use crate::{Message, MessageProcessingResult};

/// Responder error
#[derive(thiserror::Error, Debug)]
pub enum ResponderError {
    /// Communication channel error
    #[error("ResponderChannelError {0:?}")]
    ChannelError(&'static str),
    /// Response was already used
    #[error("AlreadyRespond {0}")]
    AlreadyRespond(&'static str),
}

/// Responder trait
///
/// Responsible for returning result of `Message` handling back to message sender
pub trait Responder<M: Message>: Send {
    /// Send response to the message
    ///
    /// # Errors
    ///
    /// Returns `ResponderError::AlreadyRespond` if response was already sent
    /// Returns `ResponderError::ChannelError` if communication reported an error
    fn respond(&mut self, response: MessageProcessingResult<M>) -> Result<(), ResponderError>;
}
