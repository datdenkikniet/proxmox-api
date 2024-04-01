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
}

impl FieldDef {
    pub fn new(rename: Option<String>, name: String, ty: TypeDef, optional: bool) -> Self {
        Self {
            rename,
            name,
            ty: Box::new(ty),
            optional,
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
}

impl ToTokens for FieldDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let FieldDef {
            rename,
            name,
            ty,
            optional,
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

        let default_skip = if self.optional {
            Some(quote!(#[serde(skip_serializing_if = "Option::is_none", default)]))
        } else {
            None
        };

        let ty = ty.as_field_ty(*optional);

        tokens.extend(quote! {
            #rename
            #serialize
            #default_skip
            pub #name: #ty,
        })
    }
}
