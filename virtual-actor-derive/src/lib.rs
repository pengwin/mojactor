//! Derive macroses for virtual-actor library
//! Depends on `virtual-actor` and `virtual-actor-runtime` crates

mod actor_derive;
mod message_derive;
mod virtual_actor_derive;
mod virtual_message_derive;
use proc_macro::TokenStream;

/// Derive macro for [`Actor`] trait
///
/// You can pass messages which actor can handle using attribute `message` attribute with type of message
/// For example: `#[message(TestMessage)]`
///
/// You can pass actor context using `context` attribute with type of context.
/// If you don't pass context, `virtual_actor_runtime::RuntimeContext` will be used as default.
/// For example: `#[context(TestContext)]`
#[proc_macro_derive(Actor, attributes(message, context))]
pub fn derive_actor(input: TokenStream) -> TokenStream {
    actor_derive::actor_derive(input)
}

/// Derive macro for [`Message`] trait
///
/// Requires `result` attribute with type of result.
/// For example: `#[result(Result<u64, u8>)]`
#[proc_macro_derive(Message, attributes(result))]
pub fn derive_message(input: TokenStream) -> TokenStream {
    message_derive::message_derive(input)
}

/// Derive macro for [`VirtualActor`] trait
#[proc_macro_derive(VirtualActor)]
pub fn derive_virtual_actor(input: TokenStream) -> TokenStream {
    virtual_actor_derive::virtual_actor_derive(input)
}

/// Derive macro for [`VirtualMessage`] trait
#[proc_macro_derive(VirtualMessage, attributes(result))]
pub fn derive_virtual_message(input: TokenStream) -> TokenStream {
    virtual_message_derive::virtual_message_derive(input)
}
