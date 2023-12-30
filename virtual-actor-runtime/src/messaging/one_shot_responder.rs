//! Implements responder based on tokio oneshot channel

use tokio::sync::oneshot::{channel, Receiver, Sender};
use virtual_actor::{errors::ResponderError, Message, MessageProcessingResult, Responder};

/// `Responder` based on tokio oneshot channel
pub struct OneshotResponder<M: Message> {
    /// Tokio oneshot channel sender
    sender: Option<Sender<MessageProcessingResult<M>>>,
}

impl<M: Message> OneshotResponder<M> {
    /// Create new `OneshotResponder` and return `Receiver` to wait for response
    pub fn new() -> (Self, Receiver<MessageProcessingResult<M>>) {
        let (tx, rx) = channel();
        (Self { sender: Some(tx) }, rx)
    }
}

impl<M: Message> Responder<M> for OneshotResponder<M> {
    fn respond(&mut self, response: MessageProcessingResult<M>) -> Result<(), ResponderError> {
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
