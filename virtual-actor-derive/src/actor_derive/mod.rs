//! Derive macroses for actor

mod context_attribute;
mod derive_impl;
mod message_attribute;
mod render_actor_trait_impl;
mod render_internal_mod;
mod render_message_envelope;

pub use derive_impl::actor_derive;
