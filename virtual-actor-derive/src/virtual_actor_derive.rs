//! Derive macros for virtual actor

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput, Fields};

/// Implementation of derive macro for [`VirtualActor`]
pub fn virtual_actor_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    let actor_struct: syn::DataStruct = match ast.data {
        syn::Data::Struct(data) => data,
        _ => {
            return quote_spanned! {
                ast.ident.span() =>
                compile_error!("Macro expects struct as input");
            }
            .into();
        }
    };

    let id_field_type = match get_id_field(&actor_struct.fields) {
        Some(field) => &field.ty,
        None => {
            return quote_spanned! {
                actor_struct.struct_token.span =>
                compile_error!("Struct must have `id` field");
            }
            .into();
        }
    };

    let name = &ast.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // The generated impl.

        impl ::virtual_actor_runtime::prelude::VirtualActor for #name {
            type ActorId = #id_field_type;

            fn id(&self) -> &Self::ActorId {
                &self.id
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

/// Finds `id` field required for [`VirtualActor`]
fn get_id_field(fields: &Fields) -> Option<&syn::Field> {
    match fields {
        Fields::Named(fields) => fields.named.iter().find(|f| match &f.ident {
            Some(name) => name == "id",
            None => false,
        }),
        _ => None,
    }
}
