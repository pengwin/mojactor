//! Message envelope trait

use crate::actor::Actor;

use super::{Message, MessageHandler, Responder};

/// Message envelope consumed by Actor
pub trait MessageEnvelope<A: Actor>: Send + std::fmt::Debug + Sized {}

/// Factory trait for message envelope to construct it from message type
pub trait MessageEnvelopeFactory<A, M>: MessageEnvelope<A>
where
    M: Message,
    A: MessageHandler<M>,
{
    /// Creates message envelope from message with `M` type
    fn from_message<R: Responder<M> + 'static + Sized>(msg: M, responder: Option<R>) -> Self;
}
