//! Implements responder based on tokio oneshot channel

use virtual_actor::{Message, Responder, ResponderError};

/// `Responder` based on tokio oneshot channel
pub struct OneshotResponder<M: Message> {
    /// Tokio oneshot channel sender
    sender: Option<tokio::sync::oneshot::Sender<<M as Message>::Result>>,
}

impl<M: Message> OneshotResponder<M> {
    /// Create new `OneshotResponder` and return `Receiver` to wait for response
    pub fn new() -> (Self, tokio::sync::oneshot::Receiver<<M as Message>::Result>) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        (Self { sender: Some(tx) }, rx)
    }
}

impl<M: Message> Responder<M> for OneshotResponder<M> {
    fn respond(&mut self, response: <M as Message>::Result) -> Result<(), ResponderError> {
        let tx = self.sender.take().ok_or(ResponderError::AlreadyRespond(
            "OneshotResponder already respond",
        ))?;
        if tx.send(response).is_err() {
            return Err(ResponderError::ChannelError(
                "OneshotResponder channel error. Receiver probably dropped",
            ));
        }

        Ok(())
    }
}
