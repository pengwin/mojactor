//! Virtual actors traits and types

pub mod names;

mod actor_addr;
mod actor_context_trait;
mod actor_factory_trait;
mod actor_id_trait;
mod actor_trait;
mod cancellation_token_trait;
mod mailbox_preferences;
mod message_envelope_trait;
mod message_handler_trait;
mod message_trait;
mod responder_trait;

mod local_actor_factory_trait;
mod local_actor_trait;

mod virtual_actor_factory_trait;
mod virtual_actor_trait;
mod virtual_message_trait;

/// Reexport of uuid
pub use uuid::Uuid;

// Export actor traits
pub use actor_addr::{ActorAddr, AddrError, WeakActorRef};
pub use actor_context_trait::ActorContext;
pub use actor_factory_trait::ActorFactory;
pub use actor_trait::Actor;
pub use cancellation_token_trait::CancellationToken;
pub use local_actor_factory_trait::LocalActorFactory;
pub use local_actor_trait::LocalActor;
pub use mailbox_preferences::MailboxPreferences;
pub use message_envelope_trait::{MessageEnvelope, MessageEnvelopeFactory};
pub use message_handler_trait::MessageHandler;
pub use message_trait::Message;
pub use responder_trait::Responder;
pub use responder_trait::ResponderError;

// Export virtual actor traits
pub use virtual_actor_factory_trait::VirtualActorFactory;
pub use virtual_actor_trait::VirtualActor;
pub use virtual_message_trait::VirtualMessage;
