use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

use crate::raw::KnownFormat;

use super::{
    field_def::FieldDef, proxmox_api, struct_def::AdditionalProperties, EnumDef, StructDef,
};

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
    KnownType {
        format: KnownFormat,
        fallback: PrimitiveTypeDef,
    },
    Array {
        inner: Box<TypeDef>,
    },
    Struct(StructDef),
    Enum(EnumDef),
}

impl TypeDef {
    pub const DEFAULT_DERIVES: [&'static str; 4] = [
        "Clone",
        "Debug",
        "::serde::Serialize",
        "::serde::Deserialize",
    ];

    pub fn is_unit(&self) -> bool {
        matches!(self, TypeDef::Unit)
    }

    pub fn is_array(&self) -> bool {
        matches!(self, TypeDef::Array { .. })
    }

    pub fn hoist_enum_defs(&mut self, output: &mut BTreeMap<String, EnumDef>) {
        match self {
            TypeDef::Unit => {}
            TypeDef::KnownType { .. } => {}
            TypeDef::Primitive(_) => {}
            TypeDef::Array { inner } => {
                Self::hoist_enum_defs(inner.as_mut(), output);
            }
            TypeDef::Struct(strt) => strt.hoist_enum_defs(output),
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
        values: BTreeSet<String>,
        default: Option<String>,
    ) -> Self {
        Self::Enum(EnumDef::new(name, extra_derives, values, default))
    }

    pub fn new_struct(
        name: String,
        fields: Vec<FieldDef>,
        additional_props: AdditionalProperties,
        external_defs: Vec<TypeDef>,
    ) -> Self {
        Self::Struct(StructDef::new(
            name,
            fields,
            additional_props,
            external_defs,
        ))
    }

    pub fn as_field_ty(&self, optional: bool) -> TokenStream {
        let ty = match self {
            TypeDef::Unit => quote!(()),
            TypeDef::Struct(strt) => {
                let ident = Ident::new(strt.name(), quote!().span());
                quote!(#ident)
            }
            TypeDef::KnownType { format, fallback } => Self::known_type(format, fallback),
            TypeDef::Primitive(name) => name.to_token_stream(),
            TypeDef::Array { inner } => {
                let inner = inner.as_field_ty(false);
                quote!(Vec<#inner>)
            }
            TypeDef::Enum(EnumDef { name, .. }) => {
                let ident = Ident::new(&name, quote!().span());
                quote!(#ident)
            }
        };

        if optional {
            quote!(Option<#ty>)
        } else {
            ty
        }
    }

    fn known_type(known: &KnownFormat, fallback: &PrimitiveTypeDef) -> TokenStream {
        match known {
            KnownFormat::PveVmId => proxmox_api(quote!(types::VmId)),
            _ => fallback.to_token_stream(),
        }
    }
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDef::Primitive(_) | TypeDef::Unit | TypeDef::KnownType { .. } => {}
            TypeDef::Array { inner } => inner.to_tokens(tokens),
            TypeDef::Enum(def) => def.to_tokens(tokens),
            TypeDef::Struct(strt) => strt.to_tokens(tokens),
        }
    }
}
