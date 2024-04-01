use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

use super::{field_def::FieldDef, EnumDef};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PrimitiveTypeDef {
    String,
    Number,
    Integer,
    Boolean,
}

impl ToTokens for PrimitiveTypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let this_tokens = match self {
            PrimitiveTypeDef::String => quote!(String),
            PrimitiveTypeDef::Number => quote!(f64),
            PrimitiveTypeDef::Integer => quote!(u64),
            PrimitiveTypeDef::Boolean => quote!(bool),
        };

        tokens.extend(this_tokens)
    }
}

#[derive(Clone, Debug)]
pub enum TypeDef {
    Unit,
    Primitive(PrimitiveTypeDef),
    Array {
        inner: Box<TypeDef>,
    },
    Struct {
        name: String,
        derives: Vec<String>,
        fields: Vec<FieldDef>,
        external_defs: Vec<TypeDef>,
    },
    Enum(EnumDef),
}

impl TypeDef {
    pub const DEFAULT_DERIVES: [&'static str; 4] = [
        "Clone",
        "Debug",
        "::serde::Serialize",
        "::serde::Deserialize",
    ];

    pub fn transfer_enums_in_scope(&mut self, output: &mut HashMap<String, EnumDef>) {
        match self {
            TypeDef::Unit => {}
            TypeDef::Primitive(_) => {}
            TypeDef::Array { inner } => {
                Self::transfer_enums_in_scope(inner.as_mut(), output);
            }
            TypeDef::Struct { external_defs, .. } => {
                external_defs
                    .iter_mut()
                    .for_each(|v| v.transfer_enums_in_scope(output));
                external_defs.retain(|v| !matches!(v, TypeDef::Unit))
            }
            TypeDef::Enum(def) => {
                if let Some(previous_def) = output.get_mut(&def.name) {
                    if previous_def.values != def.values {
                        eprintln!(
                            "The previous definition of enum '{}' has a different set of enum variants.",
                            def.name
                        );

                        let mut prev: Vec<_> = previous_def.values.iter().collect();
                        prev.sort();

                        let mut now: Vec<_> = def.values.iter().collect();
                        now.sort();

                        eprintln!("S1: {prev:?}");
                        eprintln!("S2: {now:?}");

                        eprintln!("Updating S1 with missing values from S2...");

                        def.values.iter().for_each(|v| {
                            previous_def.values.insert(v.clone());
                        });
                    }
                } else {
                    output.insert(def.name.to_string(), def.clone());
                }

                *self = TypeDef::Unit;
            }
        }
    }

    pub fn primitive(&self) -> Option<PrimitiveTypeDef> {
        if let Self::Primitive(p) = self {
            Some(*p)
        } else {
            None
        }
    }

    pub fn new_enum<I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: String,
        extra_derives: T,
        values: HashSet<String>,
        default: Option<String>,
    ) -> Self {
        Self::Enum(EnumDef::new(name, extra_derives, values, default))
    }

    pub fn new_struct<I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: String,
        extra_derives: T,
        fields: Vec<FieldDef>,
        external_defs: Vec<TypeDef>,
    ) -> Self {
        Self::Struct {
            name,
            derives: Self::DEFAULT_DERIVES
                .into_iter()
                .map(str::to_string)
                .chain(extra_derives.into_iter().map(|e| e.as_ref().to_string()))
                .map(|v| v.to_string())
                .collect(),
            fields,
            external_defs,
        }
    }

    pub fn as_field_ty(&self, optional: bool) -> TokenStream {
        let ty = match self {
            TypeDef::Unit => quote!(()),
            TypeDef::Struct { name, .. } => {
                let ident = Ident::new(name, quote!().span());
                quote!(#ident)
            }
            TypeDef::Primitive(name) => name.to_token_stream(),
            TypeDef::Array { inner } => {
                let inner = inner.as_field_ty(optional);
                quote!(Vec<#inner>)
            }
            TypeDef::Enum(EnumDef { name, .. }) => {
                let ident = Ident::new(name, quote!().span());
                quote!(#ident)
            }
        };

        if optional {
            quote!(Option<#ty>)
        } else {
            ty
        }
    }
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDef::Primitive(_) | TypeDef::Unit => {}
            TypeDef::Array { inner } => inner.to_tokens(tokens),
            TypeDef::Enum(def) => def.to_tokens(tokens),
            TypeDef::Struct {
                name,
                derives,
                fields,
                external_defs,
            } => {
                let name = Ident::new(name, quote!().span());

                tokens.extend(external_defs.iter().map(|def| quote! { #def }));

                let derives = derives.iter().map(|v| {
                    let parsed: TokenStream = v.parse().unwrap();
                    quote! { #parsed, }
                });

                let name = &name;

                let fields = &fields;

                tokens.extend(quote! {
                    #[derive(#(#derives)*)]
                    pub struct #name {
                        #(#fields)*
                    }
                });
            }
        }
    }
}
