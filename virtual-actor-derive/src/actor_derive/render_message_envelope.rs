//! Generates code the `MessageEnvelope` enum

use quote::{format_ident, quote_spanned, ToTokens};

use super::message_attribute::MessageAttribute;

/// Generates code the `MessageEnvelope` enum
pub fn render(
    messages: &[MessageAttribute],
    actor_name: &syn::Ident,
) -> (syn::Ident, impl ToTokens) {
    let messages_envelope_name = format_ident!("{}_MessagesEnvelope", actor_name);

    if messages.is_empty() {
        return (
            messages_envelope_name.clone(),
            quote_spanned! {
                messages_envelope_name.span() =>
                pub enum #messages_envelope_name {
                    /* Empty */
                }

                impl ::virtual_actor_runtime::prelude::MessageEnvelope<#actor_name> for #messages_envelope_name {}

                impl std::fmt::Debug for #messages_envelope_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("Empty Envelope")
                    }
                }
            },
        );
    }
    let envelope_items = messages
        .iter()
        .map(|i| i.to_enum_item(&messages_envelope_name))
        .collect::<Vec<_>>();

    let envelope_debug_items = messages
        .iter()
        .map(|i| i.to_enum_item_debug(&messages_envelope_name))
        .collect::<Vec<_>>();

    let envelope_factories = messages
        .iter()
        .map(|i| i.to_enum_factory(&messages_envelope_name, actor_name))
        .collect::<Vec<_>>();

    (
        messages_envelope_name.clone(),
        quote_spanned! {
            messages_envelope_name.span() =>
            pub enum #messages_envelope_name {
                #(#envelope_items),*
            }

            impl ::virtual_actor_runtime::prelude::MessageEnvelope<#actor_name> for #messages_envelope_name {}

            #(#envelope_factories)*

            impl std::fmt::Debug for #messages_envelope_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #(#envelope_debug_items),*
                    }
                }
            }
        },
    )
}
