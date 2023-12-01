//! Generates code for the internal mod

use quote::{format_ident, quote_spanned, ToTokens};

use super::{message_attribute::MessageAttribute, render_message_envelope};

/// Generates code for the internal mod
pub fn render(
    messages: &[MessageAttribute],
    actor_name: &syn::Ident,
) -> (syn::Ident, syn::Ident, impl ToTokens) {
    let unique_item = uuid::Uuid::new_v4().as_simple().to_string();
    let internal_mod_name = format_ident!("__{}_internal_{}", actor_name, unique_item);
    let (messages_envelope_name, rendered_envelope) =
        render_message_envelope::render(messages, actor_name);

    let mod_code = quote_spanned! {
        internal_mod_name.span() =>
        #[allow(non_snake_case)]
        mod #internal_mod_name {
            #[allow(clippy::wildcard_imports)]
            use super::*;

            /// Message envelope for #name
            #[allow(non_camel_case_types)]
            #rendered_envelope
        }
    };

    (internal_mod_name, messages_envelope_name, mod_code)
}
