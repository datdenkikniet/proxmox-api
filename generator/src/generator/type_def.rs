use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

use crate::raw::KnownFormat;

use super::{
    field_def::FieldDef, proxmox_api, struct_def::AdditionalProperties, EnumDef, NumItemsDef,
    StructDef,
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
            PrimitiveTypeDef::Integer => quote!(i64),
            PrimitiveTypeDef::Boolean => quote!(bool),
        };

        tokens.extend(this_tokens)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoundedIntegerDef {
    pub name: String,
    pub min: Option<i128>,
    pub max: Option<i128>,
    pub default: Option<i128>,
}

impl ToTokens for BoundedIntegerDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = Ident::new(&self.name, quote!().span());

        let opt_to_tokens = |opt: Option<i128>| -> TokenStream {
            match opt {
                Some(value) => quote!(Some(#value)),
                None => quote!(None::<i128>),
            }
        };

        let min = opt_to_tokens(self.min);
        let max = opt_to_tokens(self.max);
        let default = opt_to_tokens(self.default);

        let bounded_integer = proxmox_api(quote!(types::bounded_integer::BoundedInteger));
        let bounded_integer_error =
            proxmox_api(quote!(types::bounded_integer::BoundedIntegerError));
        let ser_fn = proxmox_api(quote!(types::serialize_bounded_integer));
        let des_fn = proxmox_api(quote!(types::deserialize_bounded_integer));
        let error_message = match (self.min, self.max) {
            (Some(smin), Some(max)) => format!("an integer between {} and {}", smin, max),
            (Some(min), None) => format!("an integer greater than or equal to {}", min),
            (None, Some(max)) => format!("an integer less than or equal to {}", max),
            (None, None) => "a valid integer".to_string(),
        };

        tokens.extend(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub struct #name(i128);

            impl #bounded_integer for #name {
                const MIN: Option<i128> = #min;
                const MAX: Option<i128> = #max;
                const DEFAULT: Option<i128> = #default;
                const TYPE_DESCRIPTION: &'static str = #error_message;

                fn get(&self) -> i128 {
                    self.0
                }

                fn new(value: i128) -> Result<Self, #bounded_integer_error> {
                    Self::validate(value)?;
                    Ok(Self(value))
                }
            }

            impl std::convert::TryFrom<i128> for #name {
                type Error = #bounded_integer_error;

                fn try_from(value: i128) -> Result<Self, Self::Error> {
                    #bounded_integer::new(value)
                }
            }

            impl ::serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                S: ::serde::Serializer,
                {
                    #ser_fn(self, serializer)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                D: ::serde::Deserializer<'de>,
                {
                    #des_fn(deserializer)
                }
            }
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoundedNumberDef {
    pub name: String,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub default: Option<f64>,
}

impl ToTokens for BoundedNumberDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = Ident::new(&self.name, quote!().span());

        let opt_to_tokens = |opt: Option<f64>| -> TokenStream {
            match opt {
                Some(value) => quote!(Some(#value)),
                None => quote!(None::<f64>),
            }
        };

        let min = opt_to_tokens(self.min);
        let max = opt_to_tokens(self.max);
        let default = opt_to_tokens(self.default);
        let error_message = match (self.min, self.max) {
            (Some(smin), Some(max)) => format!("an number between {} and {}", smin, max),
            (Some(min), None) => format!("an number greater than or equal to {}", min),
            (None, Some(max)) => format!("an number less than or equal to {}", max),
            (None, None) => "a valid number".to_string(),
        };

        let bounded_number = proxmox_api(quote!(types::bounded_number::BoundedNumber));
        let bounded_number_error = proxmox_api(quote!(types::bounded_number::BoundedNumberError));
        let ser_fn = proxmox_api(quote!(types::serialize_bounded_number));
        let des_fn = proxmox_api(quote!(types::deserialize_bounded_number));

        tokens.extend(quote! {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub struct #name(f64);

            impl #bounded_number for #name {
                const MIN: Option<f64> = #min;
                const MAX: Option<f64> = #max;
                const DEFAULT: Option<f64> = #default;
                const TYPE_DESCRIPTION: &'static str = #error_message;

                fn get(&self) -> f64 {
                    self.0
                }

                fn new(value: f64) -> Result<Self, #bounded_number_error> {
                    Self::validate(value)?;
                    Ok(Self(value))
                }
            }

            impl std::convert::TryFrom<f64> for #name {
                type Error = #bounded_number_error;

                fn try_from(value: f64) -> Result<Self, Self::Error> {
                    #bounded_number::new(value)
                }
            }

            impl std::convert::TryFrom<f32> for #name {
                type Error = #bounded_number_error;

                fn try_from(value: f32) -> Result<Self, Self::Error> {
                    #bounded_number::new(value as f64)
                }
            }

            impl std::convert::TryFrom<i32> for #name {
                type Error = #bounded_number_error;

                fn try_from(value: i32) -> Result<Self, Self::Error> {
                    #bounded_number::new(value as f64)
                }
            }

            impl std::convert::TryFrom<i64> for #name {
                type Error = #bounded_number_error;

                fn try_from(value: i64) -> Result<Self, Self::Error> {
                    #bounded_number::new(value as f64)
                }
            }

            impl ::serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    #ser_fn(self, serializer)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    #des_fn(deserializer)
                }
            }
        });
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoundedStringDef {
    pub name: String,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
    pub default_val: Option<String>,
}

impl ToTokens for BoundedStringDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let opt_to_tokens_usize = |opt: Option<usize>| -> TokenStream {
            match opt {
                Some(value) => quote!(Some(#value)),
                None => quote!(None::<usize>),
            }
        };
        let opt_to_tokens_str = |opt: &Option<String>| -> TokenStream {
            match opt.as_ref().map(|str| str.as_str()) {
                Some(value) => quote!(Some(#value)),
                None => quote!(None::<&'static str>),
            }
        };

        let bounded_string = quote!(crate::types::bounded_string::BoundedString);
        let name = Ident::new(&self.name, quote!().span());
        let error = quote!(crate::types::bounded_string::BoundedStringError);
        let ser_fn = proxmox_api(quote!(types::serialize_bounded_string));
        let des_fn = proxmox_api(quote!(types::deserialize_bounded_string));
        let min_len = opt_to_tokens_usize(self.min_length);
        let max_len = opt_to_tokens_usize(self.max_length);
        let default_val = opt_to_tokens_str(&self.default_val);
        let regex_pattern = opt_to_tokens_str(&self.pattern);

        let formatted_length = match (self.min_length, self.max_length) {
            (Some(min), Some(max)) => format!("length between {} and {}", min, max),
            (Some(min), None) => format!("length at least {}", min),
            (None, Some(max)) => format!("length at most {}", max),
            (None, None) => format!("no length constraints"),
        };

        let error_message = match &self.pattern {
            None => format!("a string with {}", formatted_length),
            Some(pattern) => format!(
                "a string with pattern r\"{}\" and {}",
                pattern, formatted_length
            ),
        };

        tokens.extend(quote! {
            #[derive(Debug, Clone, PartialEq, PartialOrd)]
            pub struct #name {
                value: String
            }

            impl #bounded_string for #name {
                const MIN_LENGTH: Option<usize> = #min_len;
                const MAX_LENGTH: Option<usize> = #max_len;
                const DEFAULT: Option<&'static str> = #default_val;
                const PATTERN: Option<&'static str> = #regex_pattern;
                const TYPE_DESCRIPTION: &'static str = #error_message;

                fn get_value(&self) -> &str {
                    &self.value
                }

                fn new(value: String) -> Result<Self, #error> {
                    Self::validate(&value)?;
                    Ok(Self { value })
                }
            }

            impl std::convert::TryFrom<String> for #name {
                type Error = #error;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    #bounded_string::new(value)
                }
            }

            impl ::serde::Serialize for #name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    #ser_fn(self, serializer)
                }
            }

            impl<'de> ::serde::Deserialize<'de> for #name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: ::serde::Deserializer<'de>,
                {
                    #des_fn(deserializer)
                }
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDef {
    Primitive(PrimitiveTypeDef),
    KnownType {
        format: KnownFormat,
        fallback: PrimitiveTypeDef,
    },
    Array(Box<TypeDef>),
    Struct(StructDef),
    Enum(EnumDef),
    NumberedItems(Box<NumItemsDef>),
    BoundedInteger(BoundedIntegerDef),
    BoundedNumber(BoundedNumberDef),
    BoundedString(BoundedStringDef),
}

impl TypeDef {
    pub const DEFAULT_DERIVES: [&'static str; 4] = [
        "Clone",
        "Debug",
        "::serde::Serialize",
        "::serde::Deserialize",
    ];

    pub fn is_array(&self) -> bool {
        matches!(self, TypeDef::Array(..))
    }

    pub fn primitive(&self) -> Option<PrimitiveTypeDef> {
        if let Self::Primitive(p) = self {
            Some(*p)
        } else {
            None
        }
    }

    pub fn numbered_items(&self) -> Option<&NumItemsDef> {
        if let Self::NumberedItems(n) = &self {
            Some(n)
        } else {
            None
        }
    }

    pub fn new_enum<I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: String,
        extra_derives: T,
        values: BTreeSet<String>,
        default: Option<String>,
        doc: impl Iterator<Item = String>,
    ) -> Self {
        Self::Enum(EnumDef::new(
            name,
            extra_derives,
            values,
            default,
            doc.collect(),
        ))
    }

    pub fn new_struct(
        name: String,
        fields: Vec<FieldDef>,
        additional_props: AdditionalProperties,
    ) -> Self {
        Self::Struct(StructDef::new(name, fields, additional_props))
    }

    pub fn as_field_ty(&self, optional: bool) -> (Option<&str>, TokenStream) {
        let ty = match self {
            TypeDef::NumberedItems(inner) => {
                let empty_check = "::std::collections::HashMap::is_empty";
                let (_, ty) = inner.ty().as_field_ty(false);
                let def = quote!(::std::collections::HashMap<u32, #ty>);
                return (Some(empty_check), def);
            }
            TypeDef::Struct(strt) => {
                let ident = Ident::new(&strt.name(), quote!().span());
                quote!(#ident)
            }
            TypeDef::KnownType { format, fallback } => Self::known_type(format, fallback),
            TypeDef::Primitive(name) => name.to_token_stream(),
            TypeDef::Array(inner) => {
                let (_, inner) = inner.as_field_ty(false);
                let empty_check = "::std::vec::Vec::is_empty";
                let def = quote!(Vec<#inner>);
                return (Some(empty_check), def);
            }
            TypeDef::Enum(en) => {
                let ident = Ident::new(&en.name(), quote!().span());
                quote!(#ident)
            }
            TypeDef::BoundedInteger(bounded) => {
                let ident = Ident::new(&bounded.name, quote!().span());
                quote!(#ident)
            }
            TypeDef::BoundedNumber(bounded) => {
                let ident = Ident::new(&bounded.name, quote!().span());
                quote!(#ident)
            }
            TypeDef::BoundedString(bounded) => {
                let ident = Ident::new(&bounded.name, quote!().span());
                quote!(#ident)
            }
        };

        let ty = if optional { quote!(Option<#ty>) } else { ty };
        let empty_check = optional.then_some("Option::is_none");
        (empty_check, ty)
    }

    fn known_type(known: &KnownFormat, fallback: &PrimitiveTypeDef) -> TokenStream {
        match known {
            KnownFormat::PveVmId => proxmox_api(quote!(types::VmId)),
            KnownFormat::Ipv4 => quote!(::std::net::Ipv4Addr),
            KnownFormat::Ipv6 => quote!(::std::net::Ipv6Addr),
            KnownFormat::Ip => quote!(::std::net::IpAddr),
            KnownFormat::MacAddr(allow_multicast) => {
                proxmox_api(quote!(types::MacAddr<#allow_multicast>))
            }
            _ => fallback.to_token_stream(),
        }
    }
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDef::Primitive(_) | TypeDef::KnownType { .. } | TypeDef::BoundedInteger(_) => {}
            TypeDef::BoundedNumber(def) => def.to_tokens(tokens),
            TypeDef::Array(inner) => inner.to_tokens(tokens),
            TypeDef::Enum(def) => def.to_tokens(tokens),
            TypeDef::Struct(strt) => strt.to_tokens(tokens),
            TypeDef::NumberedItems(strt) => strt.to_tokens(tokens),
            TypeDef::BoundedString(bstr) => bstr.to_tokens(tokens),
        }
    }
}
