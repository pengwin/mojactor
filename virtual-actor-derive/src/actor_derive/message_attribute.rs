//! Message attribute parser

use proc_macro::Ident;
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{Attribute, MetaList};

/// Parsed message attribute
pub struct MessageAttribute {
    /// Type of the message
    type_ident: Ident,
    /// Index of provided attribute
    index: usize,
}

impl MessageAttribute {
    /// Loads all `messages` attributes
    pub fn pase_attributes(attrs: &[Attribute]) -> Result<Vec<Self>, String> {
        attrs
            .iter()
            .filter_map(|attr| match &attr.meta {
                syn::Meta::List(meta) => {
                    if meta.path.is_ident("message") {
                        Some(meta)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .enumerate()
            .map(|(i, a)| Self::parse(a, i))
            .collect()
    }

    /// Builds item for enum
    pub fn to_enum_item(&self, indent: &syn::Ident) -> impl ToTokens {
        let index = self.index;
        let name = format_ident!("Message_{}", index);
        let msg_type = format_ident!("{}", self.type_ident.to_string());
        quote_spanned! {
            indent.span() =>
            #name(#msg_type, Option<Box<dyn ::virtual_actor_runtime::prelude::Responder<#msg_type>>>)
        }
    }

    /// Builds item for enum `std::fmt::Debug` impl
    pub fn to_enum_item_debug(&self, indent: &syn::Ident) -> impl ToTokens {
        let index = self.index;
        let name = format_ident!("Message_{}", index);
        let msg_type_str = self.type_ident.to_string();
        quote_spanned! {
            indent.span() =>
            Self::#name(_, _) => {
                f.write_str("Message_")?;
                f.write_fmt(format_args!("{}, _)", #index))?;
                f.write_str("(")?;
                f.write_str(#msg_type_str)?;
                f.write_str(", _)")
            }
        }
    }

    /// Builds enum factory for message type
    pub fn to_enum_factory(
        &self,
        enum_type_name: &syn::Ident,
        actor_name: &syn::Ident,
    ) -> impl ToTokens {
        let index = self.index;
        let name = format_ident!("Message_{}", index);
        let msg_type = format_ident!("{}", self.type_ident.to_string());
        quote_spanned! {
            enum_type_name.span() =>
            impl ::virtual_actor_runtime::prelude::MessageEnvelopeFactory<#actor_name, #msg_type> for #enum_type_name {
                fn from_message<R: ::virtual_actor_runtime::prelude::Responder<#msg_type> + 'static>(msg: #msg_type, responder: Option<R>) -> Self {
                    let responder: Option<Box<dyn ::virtual_actor_runtime::prelude::Responder<#msg_type>>> = match responder {
                        Some(r) => Some(Box::new(r)),
                        None => None,
                    };
                    Self::#name(msg, responder)
                }
            }
        }
    }

    /// Builds handler for item of message envelope
    pub fn to_enum_item_handler(&self, indent: &syn::Ident) -> impl ToTokens {
        let index = self.index;
        let name = format_ident!("Message_{}", index);
        quote_spanned! {
            indent.span() =>
            Self::MessagesEnvelope::#name(msg, responder) => {
                    let result = self.handle_with_catch(msg, ctx).await;
                    if let Some(mut responder) = responder {
                        responder.respond(result)?;
                    }
                }
        }
    }

    /// Extracts message struct from attribute, and builds identifier and type
    fn parse(attr: &MetaList, index: usize) -> Result<MessageAttribute, String> {
        let msg_type = extract_type(attr)?;
        Ok(MessageAttribute {
            type_ident: msg_type,
            index,
        })
    }
}

/// Extracts message type from attribute
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
