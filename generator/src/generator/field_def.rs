use std::ops::Deref;

use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::Ident;

use super::proxmox_api_str;
use super::type_def::PrimitiveTypeDef;
use super::NumItemsDef;
use super::TypeDef;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDef {
    rename: Option<String>,
    name: String,
    optional: bool,
    doc: Vec<String>,
    ty: Box<TypeDef>,
    numbered_items: Option<NumItemsDef>,
}

impl FieldDef {
    pub fn new<S, D>(
        name: String,
        ty: TypeDef,
        optional: bool,
        doc: D,
    ) -> (Self, Option<NumItemsDef>)
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

        // TODO: type-ify this and hoist that shit at an earlier stage
        let num_items_def = if multi {
            let def = NumItemsDef::new(&fixed_name, ty.clone());
            Some(def)
        } else {
            None
        };

        (
            Self {
                name: fixed_name,
                rename,
                ty: Box::new(ty),
                optional,
                doc: doc.map(Into::into).collect(),
                numbered_items: num_items_def.clone(),
            },
            num_items_def,
        )
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
        if self.numbered_items.is_some() {
            format!("{}s", self.name)
        } else {
            self.name.clone()
        }
    }
}

impl ToTokens for FieldDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldDef {
            rename,
            name: _,
            ty,
            optional,
            doc,
            numbered_items,
        } = self;

        let name = self.name();
        let name = Ident::new(&name, quote!().span());

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

        let serialize = if let Some(num_items) = numbered_items.as_ref() {
            let numbered_name = num_items.name();
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

        let (optional, skip_default) = if numbered_items.is_some() {
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

        let ty = ty.as_field_ty(optional); // optional = !multi && !ty.is_array() && !optional

        let ty = if numbered_items.as_ref().is_some() {
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
        tokens.extend(quote! {
            #rename
            #serialize
            #skip_default
            #(#doc)*
            pub #name: #ty,
        })
    }
}
