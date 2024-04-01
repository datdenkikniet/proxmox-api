use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident, Lit, LitStr};

use crate::{
    raw::flattened::{Collection, Node},
    PathElement, StructDef,
};

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
        let mut name = crate::name_to_struct_name(segment_name);
        name.push_str("Client");

        let name = Ident::new(&name, quote!().span());

        let methods = node.value.info.values().filter_map(|info| {
            let method = &info.method;

            let parameters = info
                .parameters
                .as_ref()
                .map(|v| {
                    let prefix = crate::name_to_struct_name(&method);
                    v.struct_def(&prefix, &node.value.path)
                })
                .flatten();

            let fn_name_str = method.to_ascii_lowercase();
            let fn_name = Ident::new(&fn_name_str, quote!().span());

            let mut returns_definitions = Vec::new();

            let returns = info
                .returns
                .as_ref()
                .map(|v| {
                    (
                        v.to_definition(&format!("{}Returns", method)),
                        v.optional.get(),
                    )
                })
                .map(|(def, optional)| {
                    let returns_def = quote! { #def };

                    returns_definitions.push(returns_def);
                    let name = def.as_field_ty(optional).clone();
                    quote! { -> Result<#name, ::proxmox_api::client::Error> }
                })
                .unwrap_or(quote! {});

            let (fn_definition, params_def) =
                if let Some(StructDef { definition, name }) = parameters {
                    let name = Ident::new(&name, quote! {}.span());

                    let def = quote! {
                        pub fn #fn_name(&self, params: #name) #returns {
                            let path = self.path.to_string();
                            self.client.#fn_name(&path, &params)
                        }
                    };

                    (def, Some(definition))
                } else {
                    let def = quote! {
                        pub fn #fn_name(&self) #returns {
                            let path = self.path.to_string();
                            self.client.#fn_name(&path, &())
                        }
                    };

                    (def, None)
                };

            let block = quote! {
                #params_def

                #(#returns_definitions)*

                impl #name {
                    #fn_definition
                }
            };

            Some(block)
        });

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

        let definition = quote! {
            pub struct #name {
                client: ::std::sync::Arc<::proxmox_api::client::Client>,
                path: String,
            }

            impl #name {
                #new
            }

            #(#methods)*

            #(#children)*
        };

        (name.to_string(), definition)
    }

    fn make_placeholder_constructor(has_parent: bool, placeholder: &str) -> TokenStream {
        let placeholder_ident = Ident::new(placeholder, quote!().span());

        if has_parent {
            quote! {
                pub fn new(client: ::std::sync::Arc<::proxmox_api::client::Client>, parent_path: &str, #placeholder_ident: &str) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, #placeholder_ident)
                    }
                }
            }
        } else {
            quote! {
                pub fn new(client: ::std::sync::Arc<::proxmox_api::client::Client>, #placeholder_ident: &str) -> Self {
                    Self {
                        client,
                        path: #placeholder_ident.to_string()
                    }
                }
            }
        }
    }

    fn make_literal_constructor(has_parent: bool, literal: &str) -> TokenStream {
        let literal = Lit::Str(LitStr::new(&format!("/{literal}"), quote!().span()));

        if has_parent {
            quote! {
                pub fn new(client: ::std::sync::Arc<::proxmox_api::client::Client>, parent_path: &str) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, #literal)
                    }
                }
            }
        } else {
            quote! {
                pub fn new(client: ::std::sync::Arc<::proxmox_api::client::Client>) -> Self {
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
