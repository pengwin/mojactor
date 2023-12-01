//! Derive macroses for actor message

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Attribute, DeriveInput, MetaList};

/// Implentation of derive macro for [`Message`]
pub fn message_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let message_struct: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _ => {
            return quote_spanned! {
                ast.ident.span() =>
                compile_error!("Macro expects struct as input");
            }
            .into();
        }
    };

    let Some(result_type) = get_result_attribute(&ast.attrs) else {
        return quote_spanned! {
            message_struct.struct_token.span =>
            compile_error!("Struct must have `result` attribute. For ex. #[result(Result<u64, u8>)]");
        }
        .into();
    };

    let tokens: proc_macro::TokenStream = result_type.tokens.clone().into();
    let tokens = parse_macro_input!(tokens as syn::Type);

    let name = &ast.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // The generated impl.

        impl ::virtual_actor_runtime::prelude::Message for #name {
            type Result = #tokens;
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

/// Finds `result` attribute required for [`VirtualMessage`]
fn get_result_attribute(attrs: &[Attribute]) -> Option<&MetaList> {
    attrs.iter().find_map(|attr| match &attr.meta {
        syn::Meta::List(meta) => {
            if meta.path.is_ident("result") {
                Some(meta)
            } else {
                None
            }
        }
        _ => None,
    })
}
