use std::ops::Deref;

use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::Ident;

use super::proxmox_api;
use super::proxmox_api_str;
use super::type_def::PrimitiveTypeDef;
use super::TypeDef;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    rename: Option<String>,
    name: String,
    ty: Box<TypeDef>,
    optional: bool,
    multi: bool,
    doc: Vec<String>,
}

impl FieldDef {
    pub fn new<S, D>(name: String, ty: TypeDef, optional: bool, doc: D) -> Self
    where
        D: Iterator<Item = S>,
        S: Into<String>,
    {
        let multi = name.ends_with("[n]");
        let fixed_name = if multi {
            crate::name_to_underscore_name(&name[..name.len() - 3])
        } else {
            crate::name_to_underscore_name(&name)
        };

        let rename = if fixed_name != name {
            Some(name.to_string())
        } else {
            None
        };

        Self {
            multi,
            name: fixed_name,
            rename,
            ty: Box::new(ty),
            optional,
            doc: doc.map(Into::into).collect(),
        }
    }

    pub fn optional(&self) -> bool {
        self.optional
            || matches!(
                self.ty.deref(),
                TypeDef::Enum(en) if en.has_default()
            )
    }

    pub fn ty(&self) -> TokenStream {
        self.ty.as_field_ty(self.optional)
    }

    pub fn name(&self) -> String {
        if self.multi {
            format!("{}s", self.name)
        } else {
            self.name.clone()
        }
    }
}

impl FieldDef {
    pub fn to_tokens(&self, top_level_defs: &mut Vec<TokenStream>) -> TokenStream {
        let FieldDef {
            rename,
            name,
            ty,
            optional,
            doc,
            multi,
        } = self;

        let original_name = name;
        let name = if *multi {
            crate::name_to_underscore_name(&format!("{name}s"))
        } else {
            crate::name_to_underscore_name(name)
        };
        let name = Ident::new(&name, quote!().span());
        let numbered_name = crate::name_to_ident(&format!("numbered_{name}"));

        let rename = if let Some(rename) = rename {
            let renamed = Literal::string(&rename);
            Some(quote!(#[serde(rename = #renamed)]))
        } else {
            None
        };

        let ser_des = |name: &str, optional: bool| {
            let name = if optional {
                format!("{name}_optional")
            } else {
                name.to_string()
            };

            let ser_fn = proxmox_api_str(format!("types::serialize_{name}"));
            let des_fn = proxmox_api_str(format!("types::deserialize_{name}"));
            Some(quote! { #[serde(serialize_with = #ser_fn, deserialize_with = #des_fn )] })
        };

        let serialize = if *multi {
            ser_des(&format!("multi::<{}, _>", numbered_name), false)
        } else if let Some(primitive) = ty.primitive() {
            let ser_des = |name: &str| ser_des(name, *optional);

            match primitive {
                PrimitiveTypeDef::String => None,
                PrimitiveTypeDef::Number => ser_des("number"),
                PrimitiveTypeDef::Integer => ser_des("int"),
                PrimitiveTypeDef::Boolean => ser_des("bool"),
            }
        } else {
            None
        };

        let (optional, skip_default) = if *multi {
            (
                false,
                Some(
                    quote!(#[serde(skip_serializing_if = "::std::collections::BTreeMap::is_empty", default)]),
                ),
            )
        } else if ty.is_array() {
            (
                false,
                Some(quote!(#[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)])),
            )
        } else if *optional {
            (
                true,
                Some(quote!(#[serde(skip_serializing_if = "Option::is_none", default)])),
            )
        } else {
            (false, None)
        };

        let ty = ty.as_field_ty(optional);

        let multi_block = if *multi {
            let num_items = proxmox_api(quote!(types::multi::NumberedItems));
            let prefix = Literal::string(&original_name);
            let numbered_name = Ident::new(&numbered_name, quote!().span());

            Some(quote! {
                #[derive(Default)]
                struct #numbered_name;

                impl #num_items for #numbered_name {
                    type Item = #ty;
                    const PREFIX: &'static str = #prefix;
                }
            })
        } else {
            None
        };

        top_level_defs.extend(multi_block);

        let ty = if *multi {
            quote!(::std::collections::BTreeMap<u32, #ty>)
        } else {
            ty
        };

        let doc = doc.iter().map(|v| {
            let v = super::clean_doc(&v);
            let doc_literal = Literal::string(&v);
            quote! {
                #[doc = #doc_literal]
            }
        });

        quote! {
            #rename
            #serialize
            #skip_default
            #(#doc)*
            pub #name: #ty,
        }
    }
}
