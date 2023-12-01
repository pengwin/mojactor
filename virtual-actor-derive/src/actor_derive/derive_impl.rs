//! Derive macro implementation for actor

use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, DeriveInput};

use super::{
    context_attribute::ContextAttribute, message_attribute::MessageAttribute,
    render_actor_trait_impl, render_internal_mod,
};

/// Implentation of derive macro for [`Message`]
pub fn actor_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let _actor_struct: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _ => {
            return quote_spanned! {
                ast.ident.span() =>
                compile_error!("Macro expects struct as input");
            }
            .into();
        }
    };

    let name = &ast.ident;

    let Ok(messages) = MessageAttribute::pase_attributes(&ast.attrs) else {
        return quote_spanned! {
            ast.ident.span() =>
            compile_error!("Unable to extract messages from struct. Use syntax ex. #[message(MessageType)]");
        }
        .into();
    };

    let context: Box<dyn ToTokens> = match ContextAttribute::pase_attribute(&ast.attrs) {
        Some(x) => match x {
            Ok(x) => Box::new(x.render(&ast.ident)),
            Err(x) => {
                return quote_spanned! {
                    ast.ident.span() =>
                    compile_error!(#x);
                }
                .into();
            }
        },
        None => Box::new(ContextAttribute::default_context_tokens(&ast.ident)),
    };

    let (internal_mod_name, messages_envelope_name, internal_mod_code) =
        render_internal_mod::render(&messages, name);

    let actor_impl_code = render_actor_trait_impl::render(
        &messages,
        name,
        &internal_mod_name,
        &messages_envelope_name,
        &context,
    );

    quote! {
        #internal_mod_code
        #actor_impl_code
    }
    .into()
}
