//! Derive macros for virtual actor

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, Attribute, DeriveInput, Fields};

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

    let id_field = pase_attributes(&ast.attrs).unwrap_or_else(|| "id".to_owned());

    let id_field_type = match get_id_field(&actor_struct.fields, &id_field) {
        Some(field) => &field.ty,
        None => {
            return syn::Error::new(
                ast.ident.span(),
                format!("Struct must have `{id_field}` field"),
            )
            .to_compile_error()
            .into();
        }
    };

    let name = &ast.ident;
    let id_field = syn::Ident::new(&id_field, name.span());

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        // The generated impl.

        impl ::virtual_actor_runtime::prelude::VirtualActor for #name {
            type ActorId = #id_field_type;

            fn id(&self) -> &Self::ActorId {
                &self.#id_field
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

/// Finds `id` field required for [`VirtualActor`]
fn get_id_field<'a>(fields: &'a Fields, id_field: &'a str) -> Option<&'a syn::Field> where {
    match fields {
        Fields::Named(fields) => fields.named.iter().find(|f| match &f.ident {
            Some(name) => name == id_field,
            None => false,
        }),
        _ => None,
    }
}

fn pase_attributes(attrs: &[Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| match &attr.meta {
        syn::Meta::List(meta) => {
            if meta.path.is_ident("id_field") {
                parse_id_field_attr(meta)
            } else {
                None
            }
        }
        _ => None,
    })
}

fn parse_id_field_attr(attr: &syn::MetaList) -> Option<String> {
    let tokens: proc_macro::TokenStream = attr.tokens.clone().into();
    tokens.into_iter().find_map(|t| match t {
        proc_macro::TokenTree::Ident(ty) => Some(ty.to_string()),
        _ => None,
    })
}
