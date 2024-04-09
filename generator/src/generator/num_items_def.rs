use proc_macro2::Literal;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

use super::{proxmox_api, TypeDef};

#[derive(Clone, Debug, PartialEq)]
pub struct NumItemsDef {
    prefix: String,
    ty: TypeDef,
}

impl NumItemsDef {
    pub fn new(prefix: &str, ty: TypeDef) -> Self {
        Self {
            prefix: prefix.to_string(),
            ty,
        }
    }

    pub fn name(&self) -> String {
        crate::name_to_ident(&format!("numbered_{}s", self.prefix))
    }

    pub fn ty(&self) -> &TypeDef {
        &self.ty
    }
}

impl ToTokens for NumItemsDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { prefix, ty } = self;

        let num_items = proxmox_api(quote!(types::multi::NumberedItems));
        let name = self.name();
        let name = Ident::new(&name, quote!().span());
        let prefix = Literal::string(&prefix);
        let (_, ty) = ty.as_field_ty(false);

        tokens.extend(quote! {
            #[derive(Default)]
            struct #name;

            impl #num_items for #name {
                type Item = #ty;
                const PREFIX: &'static str = #prefix;
            }
        });
    }
}
