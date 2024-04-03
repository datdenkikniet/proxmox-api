use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::Ident;

use super::proxmox_api_str;
use super::type_def::PrimitiveTypeDef;
use super::EnumDef;
use super::TypeDef;

#[derive(Debug, Clone)]
pub struct FieldDef {
    rename: Option<String>,
    name: String,
    ty: Box<TypeDef>,
    optional: bool,
    doc: Vec<String>,
}

impl FieldDef {
    pub fn new<S, D>(name: String, ty: TypeDef, optional: bool, doc: D) -> Self
    where
        D: Iterator<Item = S>,
        S: Into<String>,
    {
        let fixed_name = crate::name_to_underscore_name(&name);

        let rename = if fixed_name != name {
            Some(name.to_string())
        } else {
            None
        };

        Self {
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
                *self.ty,
                TypeDef::Enum(EnumDef {
                    default: Some(_),
                    ..
                })
            )
    }

    pub fn ty(&self) -> TokenStream {
        self.ty.as_field_ty(self.optional)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl ToTokens for FieldDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldDef {
            rename,
            name,
            ty,
            optional,
            doc,
        } = self;

        let name = crate::name_to_underscore_name(name);
        let name = Ident::new(&name, quote!().span());

        let rename = if let Some(rename) = rename {
            let renamed = Literal::string(&rename);
            Some(quote!(#[serde(rename = #renamed)]))
        } else {
            None
        };

        let serialize = if let Some(primitive) = ty.primitive() {
            let ser_des = |primitive: &str| {
                let name = if *optional {
                    format!("{primitive}_optional")
                } else {
                    primitive.to_string()
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

        let (optional, skip_default) = if ty.is_array() {
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
