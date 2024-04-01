use std::collections::HashSet;

use proc_macro2::{Literal, TokenStream};

use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

#[derive(Clone, Debug)]
pub struct EnumDef {
    pub name: String,
    pub derives: Vec<String>,
    pub values: HashSet<String>,
    pub default: Option<String>,
}

impl ToTokens for EnumDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            derives,
            values,
            default,
        } = self;

        let name = Ident::new(name, quote!().span());

        let derives = derives.iter().map(|v| {
            let parsed: TokenStream = v.parse().unwrap();
            quote! { #parsed, }
        });

        let variants = values.iter().map(|orig| {
            let v = if orig.chars().next().unwrap().is_numeric() {
                format!("_{orig}")
            } else {
                orig.to_string()
            };

            let v = crate::name_to_ident(&v);

            let rename = if orig != &v {
                let orig = Literal::string(&orig);
                Some(quote!(#[serde(rename = #orig)]))
            } else {
                None
            };

            let ident = Ident::new(&v, quote!().span());

            quote! {
                #rename
                #ident,
            }
        });

        tokens.extend(quote! {
            #[derive(#(#derives)*)]
            pub enum #name {
                #(#variants)*
            }
        });

        if let Some(default) = default {
            let default_ident = Ident::new(&default, quote!().span());

            tokens.extend(quote! {
                impl Default for #name {
                    fn default() -> Self {
                        Self::#default_ident
                    }
                }
            })
        }
    }
}
