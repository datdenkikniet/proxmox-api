use std::sync::Arc;

use parking_lot::Mutex;
use proc_macro2::Literal;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

use super::{proxmox_api, TypeDef};

#[derive(Clone, Debug)]
pub struct NumItemsDef {
    prefix: String,
    ty: TypeDef,
    name: Arc<Mutex<String>>,
}

impl PartialEq for NumItemsDef {
    fn eq(&self, other: &Self) -> bool {
        self.prefix == other.prefix
            && self.ty == other.ty
            && *self.name.lock() == *other.name.lock()
    }
}

impl NumItemsDef {
    pub fn new(prefix: &str, ty: TypeDef) -> Self {
        let name = crate::name_to_ident(&format!("numbered_{}s", prefix));

        Self {
            prefix: prefix.to_string(),
            ty,
            name: Arc::new(Mutex::new(name)),
        }
    }

    pub fn name(&self) -> String {
        self.name.lock().clone()
    }

    pub fn set_name(&mut self, name: &str) {
        let mut my_name = self.name.lock();

        if *my_name != name {
            *my_name = name.to_string();
        }
    }

    pub fn ty(&self) -> &TypeDef {
        &self.ty
    }
}

impl ToTokens for NumItemsDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { prefix, ty, name } = self;

        let num_items = proxmox_api(quote!(types::multi::NumberedItems));
        let name = Ident::new(&name.lock(), quote!().span());
        let prefix = Literal::string(prefix);
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
