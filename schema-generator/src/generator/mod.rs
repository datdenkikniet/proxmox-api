use std::collections::HashMap;

use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

use crate::{
    raw::flattened::{Collection, Node},
    PathElement,
};

mod enum_def;
pub(crate) use enum_def::EnumDef;

mod field_def;
pub(crate) use field_def::FieldDef;

mod type_def;
pub(crate) use type_def::{PrimitiveTypeDef, TypeDef};

pub(self) fn proxmox_api(path: TokenStream) -> TokenStream {
    quote! { ::proxmox_api::#path }
}

pub(self) fn proxmox_api_str(path: String) -> Literal {
    Literal::string(&format!("::proxmox_api::{path}"))
}

pub struct Generator<'a> {
    collection: Collection<'a>,
}

impl<'a> Generator<'a> {
    pub fn new(collection: Collection<'a>) -> Self {
        Self { collection }
    }

    pub fn generate(&self, stream: &mut TokenStream) {
        for node in self.collection.iter() {
            let (_, client_code) = Self::generate_client(None, node);

            let mod_name = node
                .value
                .path
                .iter()
                .next()
                .unwrap()
                .as_string_without_braces();
            let mod_ident = Ident::new(mod_name, quote!().span());

            stream.extend(quote! { pub mod #mod_ident { #client_code } });
        }
    }

    fn generate_client(parent: Option<&Node>, node: &Node) -> (String, TokenStream) {
        let path = node.value.path.iter().last().unwrap();
        Self::generate_client_impl(parent.is_some(), node, path)
    }

    fn generate_client_impl(
        has_parent: bool,
        node: &Node,
        segment: &PathElement,
    ) -> (String, TokenStream) {
        let segment_name = segment.as_string_without_braces();
        let mut name = crate::name_to_ident(segment_name);
        name.push_str("Client");

        let name = Ident::new(&name, quote!().span());

        let mut enums = HashMap::new();

        let methods: Vec<_> = node.value.info.values().filter_map(|info| {
            let method = &info.method;

            let mut parameters = info
                .parameters
                .as_ref()
                .map(|v| {
                    let prefix = crate::name_to_ident(&method);
                    let mut type_def = v.type_def(&prefix, &node.value.path);

                    if let Some(def) = type_def.as_mut() {
                        def.transfer_enums_in_scope(&mut enums);
                    }

                    type_def
                })
                .flatten();

            let fn_name_str = method.to_ascii_lowercase();
            let fn_name = Ident::new(&fn_name_str, quote!().span());

            if let Some(parameters) = &mut parameters {
                parameters.transfer_enums_in_scope(&mut enums);
            }

            let (signature, defer_signature, params_def) =
                if let Some(TypeDef::Struct { name, .. }) = &parameters {
                    let name = Ident::new(name, quote! {}.span());
                    (
                        quote!(&self, params: #name),
                        quote!(&path, &params),
                        Some(quote! { #parameters }),
                    )
                } else {
                    (quote!(&self), quote!(&path, &()), None)
                };

            let (returns, returns_definition, call) = if let Some(ret) = info.returns.as_ref() {
                let mut def = ret.type_def("", &format!("{method}Output"));

                def.transfer_enums_in_scope(&mut enums);

                let returns_def = quote! { #def };

                let name = def.as_field_ty(ret.optional.get());
                let error = proxmox_api(quote!(Error));

                let call = match def.primitive() {
                    Some(PrimitiveTypeDef::Integer) => {
                        let int = proxmox_api(quote!(Integer));
                        quote!(Ok(self.client.#fn_name::<_, _, #int>(#defer_signature)?.get()))
                    }
                    Some(PrimitiveTypeDef::Number) => {
                        let number = proxmox_api(quote!(Number));
                        quote!(Ok(self.client.#fn_name::<_, _, #number>(#defer_signature)?.get()))
                    }
                    Some(PrimitiveTypeDef::Boolean) => {
                        let bool = proxmox_api(quote!(Bool));
                        quote!(Ok(self.client.#fn_name::<_, _, #bool>(#defer_signature)?.get()))
                    }
                    Some(PrimitiveTypeDef::String) | None => {
                        quote!(self.client.#fn_name(#defer_signature))
                    }
                };

                (quote! { -> Result<#name, #error> }, Some(returns_def), call)
            } else {
                (
                    quote!(),
                    None,
                    quote!(self.client.#fn_name(#defer_signature)),
                )
            };

            let fn_definition = quote! {
                pub fn #fn_name(#signature) #returns {
                    let path = self.path.to_string();
                    #call
                }
            };

            let block = quote! {
                #params_def

                #returns_definition

                impl #name {
                    #fn_definition
                }
            };

            Some(block)
        }).collect();

        let children = node.children.iter().map(|(segment, child)| {
            assert_eq!(
                node.value.path.iter().count() + 1,
                child.value.path.iter().count(),
                "Child node is not a direct child of its parent, path-wise"
            );

            let segment_no_braces =
                crate::name_to_underscore_name(segment.as_string_without_braces());
            let mod_name = Ident::new(&segment_no_braces, quote!().span());

            let (child_name, child_data) = Self::generate_client(Some(node), child);
            let child_name = Ident::new(&child_name, quote!().span());
            let defer = quote! { #mod_name::#child_name };
            let child_fn_name = Ident::new(&segment_no_braces, quote!().span());

            let (new_sig, child_call) = match segment {
                PathElement::Literal(_) => (
                    quote! { #child_fn_name(&self) -> #defer },
                    quote! { #defer::new(self.client.clone(), &self.path) },
                ),
                PathElement::Placeholder(p) => {
                    let next_val_ident = Ident::new(p, quote!().span());
                    (
                        quote! { #child_fn_name(&self, #next_val_ident: &str) -> #defer },
                        quote! { #defer::new(self.client.clone(), &self.path, #next_val_ident)},
                    )
                }
            };

            quote! {
                pub mod #mod_name {
                    #child_data
                }

                impl #name {
                    pub fn #new_sig {
                        #child_call
                    }
                }
            }
        });

        let new = match segment {
            PathElement::Literal(literal) => Self::make_literal_constructor(has_parent, literal),
            PathElement::Placeholder(placeholder) => {
                Self::make_placeholder_constructor(has_parent, placeholder)
            }
        };

        let client = proxmox_api(quote!(Client));
        let enums = enums.values();
        let definition = quote! {
            pub struct #name {
                client: ::std::sync::Arc<#client>,
                path: String,
            }

            impl #name {
                #new
            }

            #(#enums)*

            #(#methods)*

            #(#children)*
        };

        (name.to_string(), definition)
    }

    fn make_placeholder_constructor(has_parent: bool, placeholder: &str) -> TokenStream {
        let placeholder_ident = Ident::new(placeholder, quote!().span());
        let client = proxmox_api(quote!(Client));

        if has_parent {
            quote! {
                pub fn new(client: ::std::sync::Arc<#client>, parent_path: &str, #placeholder_ident: &str) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, #placeholder_ident)
                    }
                }
            }
        } else {
            quote! {
                pub fn new(client: ::std::sync::Arc<#client>, #placeholder_ident: &str) -> Self {
                    Self {
                        client,
                        path: #placeholder_ident.to_string()
                    }
                }
            }
        }
    }

    fn make_literal_constructor(has_parent: bool, literal: &str) -> TokenStream {
        let literal = Literal::string(&format!("/{literal}"));
        let client = proxmox_api(quote!(Client));

        if has_parent {
            quote! {
                pub fn new(client: ::std::sync::Arc<#client>, parent_path: &str) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, #literal)
                    }
                }
            }
        } else {
            quote! {
                pub fn new(client: ::std::sync::Arc<#client>) -> Self {
                    Self {
                        client,
                        path: #literal.to_string()
                    }
                }
            }
        }
    }
}

impl ToTokens for Generator<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.generate(tokens);
    }
}
