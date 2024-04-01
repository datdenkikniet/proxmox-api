use std::collections::HashSet;

use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::Ident;
use syn::Lit;

use super::proxmox_api_str;

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub rename: Option<String>,
    pub name: String,
    pub ty: TokenStream,
    pub optional: bool,
    pub primitive_ty: Option<PrimitiveTypeDef>,
}

impl ToTokens for FieldDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldDef {
            rename,
            name,
            ty,
            optional,
            primitive_ty,
        } = self;

        let name = Ident::new(name, quote!().span());

        let rename = if let Some(rename) = rename {
            let renamed = Lit::new(Literal::string(&rename));
            Some(quote!(#[serde(rename = #renamed)]))
        } else {
            None
        };

        let serialize = if let Some(primitive) = primitive_ty {
            let ser_des = |name: &str| {
                let name = if *optional {
                    format!("{name}_optional")
                } else {
                    name.to_string()
                };

                let ser_fn = proxmox_api_str(format!("serialize_{name}"));
                let des_fn = proxmox_api_str(format!("deserialize_{name}"));
                Some(quote! { #[serde(serialize_with = #ser_fn, deserialize_with = #des_fn )] })
            };

            match primitive {
                PrimitiveTypeDef::String => None,
                PrimitiveTypeDef::Number => ser_des("number"),
                PrimitiveTypeDef::Integer => ser_des("int"),
                PrimitiveTypeDef::Boolean => ser_des("bool"),
            }
        } else {
            None
        };

        let default_skip = if self.optional {
            Some(quote!(#[serde(skip_serializing_if = "Option::is_none", default)]))
        } else {
            None
        };

        tokens.extend(quote! {
            #rename
            #serialize
            #default_skip
            pub #name: #ty,
        })
    }
}

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

#[derive(Clone)]
pub enum TypeDef {
    Unit,
    Primitive(PrimitiveTypeDef),
    Array {
        inner: Box<TypeDef>,
    },
    Struct {
        name: TokenStream,
        derives: Vec<String>,
        fields: Vec<FieldDef>,
        external_defs: Vec<TypeDef>,
    },
    Enum {
        name: TokenStream,
        derives: Vec<String>,
        values: HashSet<String>,
        default: Option<String>,
    },
}

impl TypeDef {
    pub const DEFAULT_DERIVES: [&'static str; 4] = [
        "Clone",
        "Debug",
        "::serde::Serialize",
        "::serde::Deserialize",
    ];

    pub fn primitive(&self) -> Option<PrimitiveTypeDef> {
        if let Self::Primitive(p) = self {
            Some(*p)
        } else {
            None
        }
    }

    pub fn new_enum<I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: TokenStream,
        extra_derives: T,
        values: HashSet<String>,
        default: Option<String>,
    ) -> Self {
        if let Some(default) = default.as_ref() {
            assert!(values.contains(default));
        }

        Self::Enum {
            name,
            derives: Self::DEFAULT_DERIVES
                .into_iter()
                .map(str::to_string)
                .chain(extra_derives.into_iter().map(|e| e.as_ref().to_string()))
                .map(|v| v.to_string())
                .collect(),
            values,
            default,
        }
    }

    pub fn new_struct<I: AsRef<str>, T: IntoIterator<Item = I>>(
        name: TokenStream,
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
            TypeDef::Struct { name, .. } => name.clone(),
            TypeDef::Primitive(name) => name.to_token_stream(),
            TypeDef::Array { inner } => {
                let inner = inner.as_field_ty(optional);
                quote!(Vec<#inner>)
            }
            TypeDef::Enum { name, .. } => name.clone(),
        };

        if optional {
            quote!(Option<#ty>)
        } else {
            ty
        }
    }

    pub fn is_enum_with_default(&self) -> bool {
        match self {
            TypeDef::Enum { default, .. } => default.is_some(),
            _ => false,
        }
    }
}

impl ToTokens for TypeDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeDef::Primitive(_) | TypeDef::Unit => {}
            TypeDef::Array { inner } => inner.to_tokens(tokens),
            TypeDef::Enum {
                name,
                values,
                default,
                derives,
            } => {
                let derives = derives.iter().map(|v| {
                    let parsed: TokenStream = v.parse().unwrap();
                    quote! { #parsed, }
                });

                let variants = values.iter().map(|v| Ident::new(&v, quote!().span()));

                tokens.extend(quote! {
                    #[derive(#(#derives)*)]
                    pub enum #name {
                        #(#variants,)*
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
            TypeDef::Struct {
                name,
                derives,
                fields,
                external_defs,
            } => {
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
