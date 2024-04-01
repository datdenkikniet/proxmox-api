use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Ident};

use super::{EnumDef, FieldDef, TypeDef};

use quote::quote;

#[derive(Clone, Debug)]
pub struct StructDef {
    name: String,
    fields: Vec<FieldDef>,
    external_defs: Vec<TypeDef>,
}

impl StructDef {
    pub fn new(name: String, fields: Vec<FieldDef>, external_defs: Vec<TypeDef>) -> Self {
        Self {
            name,
            fields,
            external_defs,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[FieldDef] {
        &self.fields
    }

    pub fn external_defs(&self) -> &[TypeDef] {
        &self.external_defs
    }

    pub fn hoist_enum_defs(&mut self, output: &mut HashMap<String, EnumDef>) {
        self.external_defs
            .iter_mut()
            .for_each(|v| v.hoist_enum_defs(output));
        self.external_defs.retain(|v| !matches!(v, TypeDef::Unit))
    }
}

impl ToTokens for StructDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            name,
            fields,
            external_defs,
        } = self;

        let name = Ident::new(name, quote!().span());

        tokens.extend(external_defs.iter().map(|def| quote! { #def }));

        let default_derive = if fields.iter().all(|f| f.optional()) {
            Some("Default")
        } else {
            let optional_fields = fields.iter().filter(|f| f.optional());
            let non_optional_fields = fields.iter().filter(|f| !f.optional());

            let default_fields = optional_fields.map(|f| {
                let name = f.name();
                let name = Ident::new(name, quote!().span());
                quote!(#name: Default::default())
            });

            let args = non_optional_fields.clone().map(|f| {
                let name = f.name();
                let name = Ident::new(name, quote!().span());
                let ty = f.ty();
                quote!(#name: #ty)
            });

            let arg_setters = non_optional_fields.map(|f| {
                let name = f.name();
                let name = Ident::new(name, quote!().span());
                quote!(#name)
            });

            tokens.extend(quote! {
                impl #name {
                    pub fn new(#(#args),*) -> Self {
                        Self {
                            #(#arg_setters),*,
                            #(#default_fields),*
                        }
                    }
                }
            });

            None
        };

        let derives = TypeDef::DEFAULT_DERIVES
            .iter()
            .chain(default_derive.iter())
            .map(|v| {
                let parsed: TokenStream = v.parse().unwrap();
                quote! { #parsed, }
            });

        let name = &name;

        tokens.extend(quote! {
            #[derive(#(#derives)*)]
            pub struct #name {
                #(#fields)*
            }
        });
    }
}
