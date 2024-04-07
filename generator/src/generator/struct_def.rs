use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Ident};

use super::{proxmox_api, proxmox_api_str, FieldDef, TypeDef};

use quote::quote;

#[derive(Clone, Debug, PartialEq)]
pub enum AdditionalProperties {
    None,
    Untyped,
    Type(Box<TypeDef>),
}

impl AdditionalProperties {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructDef {
    name: String,
    fields: Vec<FieldDef>,
    additional_props: AdditionalProperties,
}

impl StructDef {
    pub fn new(
        name: String,
        fields: Vec<FieldDef>,
        additional_props: AdditionalProperties,
    ) -> Self {
        Self {
            name,
            fields,
            additional_props,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[FieldDef] {
        &self.fields
    }
}

impl ToTokens for StructDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            name: _,
            fields,
            additional_props,
        } = self;

        let name = Ident::new(self.name(), quote!().span());

        let additional_props_ty = match additional_props {
            AdditionalProperties::None => None,
            AdditionalProperties::Untyped => {
                Some(quote!(::std::collections::HashMap<String, ::serde_json::Value>))
            }
            AdditionalProperties::Type(ty) => {
                ty.to_tokens(tokens);

                let ty = ty.as_field_ty(false);
                Some(quote! (::std::collections::HashMap<String, #ty>))
            }
        };

        let default_derive = if fields.iter().all(|f| f.optional()) {
            Some("Default")
        } else {
            let optional_fields = fields.iter().filter(|f| f.optional());
            let non_optional_fields = fields.iter().filter(|f| !f.optional());

            let default_fields = optional_fields.map(|f| {
                let name = f.name();
                let name = Ident::new(&name, quote!().span());
                quote!(#name: Default::default())
            });

            let args = non_optional_fields.clone().map(|f| {
                let name = f.name();
                let name = Ident::new(&name, quote!().span());
                let ty = f.ty();
                quote!(#name: #ty)
            });

            let arg_setters = non_optional_fields.map(|f| {
                let name = f.name();
                let name = Ident::new(&name, quote!().span());
                quote!(#name)
            });

            let additional_props = additional_props_ty
                .as_ref()
                .map(|_| quote!(additional_properties: Default::default(),))
                .into_iter();

            tokens.extend(quote! {
                impl #name {
                    pub fn new(#(#args),*) -> Self {
                        Self {
                            #(#arg_setters,)*
                            #(#default_fields,)*
                            #(#additional_props),*
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

        let (additional_props, test) = if let Some(additional_props) = additional_props_ty {
            let multis: Vec<_> = fields
                .iter()
                .filter_map(|f| f.multi())
                .map(|m| m.name())
                .collect();

            let (serde_attr, test) = if multis.is_empty() {
                let test = None;
                let serde = quote! {
                    #[serde(flatten, default, skip_serializing_if = "::std::collections::HashMap::is_empty")]
                };
                (serde, test)
            } else {
                let idents = multis.into_iter().map(|v| Ident::new(&v, quote!().span()));
                let test = proxmox_api(quote!(types::multi::Test));
                let numbered_items = proxmox_api(quote!(types::multi::NumberedItems));
                let test = Some(quote! {
                    impl #test for #name {
                        fn test_fn() -> fn(&str) -> bool {
                            fn the_test(input: &str) -> bool {
                                let array = [
                                    #(<#idents as #numbered_items>::key_matches as fn(&str) -> bool,)*
                                ];

                                array.iter().any(|f| f(input))
                            }

                            the_test as _
                        }
                    }
                });

                let path = proxmox_api_str(format!(
                    "types::multi::deserialize_additional_data::<'_, {name}, _, _>"
                ));
                let serde = quote! {
                    #[serde(flatten, deserialize_with = #path)]
                };
                (serde, test)
            };

            let additional_props = quote! {
                #serde_attr
                pub additional_properties: #additional_props,
            };

            (Some(additional_props), test)
        } else {
            (None, None)
        };

        tokens.extend(quote! {
            #[derive(#(#derives)*)]
            pub struct #name {
                #(#fields)*
                #additional_props
            }

            #test
        });
    }
}
