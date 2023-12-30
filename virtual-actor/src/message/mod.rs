//! Module with message traits and types

mod mailbox_preferences;
mod message_envelope_trait;
mod message_handler_trait;
mod message_name;
mod message_processing_result;
mod message_trait;
mod responder_trait;

pub use mailbox_preferences::MailboxPreferences;
pub use message_envelope_trait::{MessageEnvelope, MessageEnvelopeFactory};
pub use message_handler_trait::MessageHandler;
pub use message_name::MessageName;
pub use message_processing_result::MessageProcessingResult;
pub use message_trait::Message;
pub use responder_trait::Responder;
