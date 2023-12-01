//! Generates implementation for `Actor` trait

use quote::{quote_spanned, ToTokens};

use super::message_attribute::MessageAttribute;

/// Generates code for the internal mod
pub fn render(
    messages: &[MessageAttribute],
    actor_name: &syn::Ident,
    internal_mod_name: &syn::Ident,
    messages_envelope_name: &syn::Ident,
    context_attribute: &dyn ToTokens,
) -> impl ToTokens {
    let rendered_envelope_handler = if messages.is_empty() {
        quote_spanned! {
            actor_name.span() =>
            async fn handle_envelope(
                &mut self,
                _envelope: Self::MessagesEnvelope,
                _ctx: &Self::ActorContext,
            ) -> Result<(), ::virtual_actor_runtime::prelude::ResponderError> {
                Ok(())
            }
        }
    } else {
        let envelope_handlers = messages
            .iter()
            .map(|i| i.to_enum_item_handler(actor_name))
            .collect::<Vec<_>>();

        quote_spanned! {
            actor_name.span() =>
            async fn handle_envelope(
                &mut self,
                envelope: Self::MessagesEnvelope,
                ctx: &Self::ActorContext,
            ) -> Result<(), ::virtual_actor_runtime::prelude::ResponderError> {
                match envelope {
                    #(#envelope_handlers),*
                }
                Ok(())
            }
        }
    };

    quote_spanned! {
        actor_name.span() =>
        /// Actor implementation for #name
        impl ::virtual_actor_runtime::prelude::Actor for #actor_name {

            type ActorContext = #context_attribute;

            type MessagesEnvelope = #internal_mod_name::#messages_envelope_name;

            #rendered_envelope_handler
        }
    }
}
