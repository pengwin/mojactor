//! Context attribute parser

use proc_macro::Ident;
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{Attribute, MetaList};

/// Parsed context attribute
pub struct ContextAttribute {
    /// Context type
    pub context_type: Ident,
}

impl ContextAttribute {
    /// Loads `context` attribute
    pub fn pase_attribute(attrs: &[Attribute]) -> Option<Result<ContextAttribute, String>> {
        attrs.iter().find_map(|attr| match &attr.meta {
            syn::Meta::List(meta) => {
                if meta.path.is_ident("context") {
                    Some(Self::parse(meta))
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    /// Builds default context
    pub fn default_context_tokens(actor_name: &syn::Ident) -> impl ToTokens {
        quote_spanned!(actor_name.span() => ::virtual_actor_runtime::RuntimeContext<#actor_name>)
    }

    /// Render context type code
    pub fn render(self, actor_name: &syn::Ident) -> impl ToTokens {
        let context_type = self.context_type;
        let ident = format_ident!("{}", context_type.to_string());
        quote_spanned! {
            actor_name.span() =>
            #ident
        }
    }

    /// Extracts context struct from attribute, and builds identifier and type
    fn parse(attr: &MetaList) -> Result<ContextAttribute, String> {
        let ctx_type = extract_type(attr)?;
        Ok(ContextAttribute {
            context_type: ctx_type,
        })
    }
}

/// Extracts context type from attribute
fn extract_type(attr: &MetaList) -> Result<Ident, String> {
    let tokens: proc_macro::TokenStream = attr.tokens.clone().into();
    tokens
        .into_iter()
        .find_map(|t| match t {
            proc_macro::TokenTree::Ident(ty) => Some(ty),
            _ => None,
        })
        .map_or_else(|| Err("Unable to find ident type".to_string()), Ok)
}
