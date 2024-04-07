use std::collections::BTreeSet;

use proc_macro2::{Literal, TokenStream};

use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

#[derive(Clone, Debug, PartialEq)]
pub struct EnumDef {
    name: String,
    derives: Vec<String>,
    values: BTreeSet<String>,
    default: Option<String>,
}

impl EnumDef {
    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }

    pub fn values(&self) -> &BTreeSet<String> {
        &self.values
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn values_mut(&mut self) -> &mut BTreeSet<String> {
        &mut self.values
    }

    pub fn new<I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: String,
        extra_derives: T,
        values: BTreeSet<String>,
        default: Option<String>,
    ) -> Self {
        if let Some(default) = default.as_ref() {
            assert!(values.contains(default));
        }

        let me = EnumDef {
            name,
            derives: super::TypeDef::DEFAULT_DERIVES
                .into_iter()
                .map(str::to_string)
                .chain(extra_derives.into_iter().map(|e| e.as_ref().to_string()))
                .map(|v| v.to_string())
                .collect(),
            values,
            default,
        };

        me
    }

    fn fix_name(name: &str) -> String {
        if name.chars().next().unwrap().is_numeric() {
            format!("_{name}")
        } else {
            name.to_string()
        }
    }
}

impl ToTokens for EnumDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name: _,
            derives,
            values,
            default,
        } = self;

        let name = Ident::new(self.name(), quote!().span());

        let derives = derives.iter().map(|v| {
            let parsed: TokenStream = v.parse().unwrap();
            quote! { #parsed, }
        });

        let variants = values.iter().map(|orig| {
            let v = Self::fix_name(orig);
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
            let default = Self::fix_name(default);
            let default = crate::name_to_ident(&default);
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
