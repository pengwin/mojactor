//! Derive macroses for virtual message

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};

/// Implentation of derive macro for [`VirtualMessage`]
pub fn virtual_message_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let _message_struct: syn::DataStruct = match ast.data {
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

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // The generated impl.

        impl ::virtual_actor_runtime::prelude::VirtualMessage for #name {
            fn name() -> ::virtual_actor_runtime::prelude::names::MessageName {
                stringify!(#name)
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
