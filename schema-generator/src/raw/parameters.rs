use std::{borrow::Cow, collections::HashMap};

use serde::Deserialize;
use syn::{spanned::Spanned, Ident};

use quote::quote;

use crate::{struct_def::StructDef, Path, PathElement};

use super::Type;

#[derive(Debug, Clone, Deserialize)]
pub struct Parameters<'a> {
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<Cow<'a, str>, Type<'a>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}

impl Parameters<'_> {
    pub fn struct_def(&self, prefix: &str, path: &Path) -> Option<StructDef> {
        if let Some(properties) = &self.properties {
            let name = format!("{prefix}Params");
            let ident = Ident::new(&name, quote::quote! {}.span());

            let optional = properties.values().all(|prop| prop.optional.get());

            let fields = properties.iter().filter_map(|(name, ty)| {
                if path.iter().any(|p| {
                    if let PathElement::Placeholder(placeholder) = p {
                        placeholder == name
                    } else {
                        false
                    }
                }) {
                    return None;
                }

                let ty = ty.to_tokens();
                let field_ident = Ident::new(&name.replace("-", "_"), quote::quote! {}.span());

                Some(quote::quote! {
                    pub #field_ident: #ty,
                })
            });

            let derive = if optional {
                quote! { #[derive(::std::fmt::Debug, ::serde::Serialize, Default)] }
            } else {
                quote! { #[derive(::std::fmt::Debug, ::serde::Serialize)] }
            };

            let fields: Vec<_> = fields.collect();

            if fields.is_empty() {
                return None;
            }

            let definition = quote::quote! {
                #derive
                pub struct #ident {
                    #(#fields)*
                }
            };

            Some(StructDef { name, definition })
        } else {
            None
        }
    }
}
