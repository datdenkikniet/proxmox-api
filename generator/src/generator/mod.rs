use std::collections::BTreeMap;

use proc_macro2::{Literal, Punct, TokenStream};
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

mod file;

mod struct_def;
pub(crate) use struct_def::{AdditionalProperties, StructDef};

mod mod_def;

mod type_def;
pub(crate) use type_def::{PrimitiveTypeDef, TypeDef};

pub use self::mod_def::ClientModDef;

pub(self) fn proxmox_api(path: TokenStream) -> TokenStream {
    quote! { crate::#path }
}

pub(self) fn proxmox_api_str(path: String) -> Literal {
    Literal::string(&format!("crate::{path}"))
}

pub fn clean_doc(input: &str) -> String {
    input.replace('<', "\\<").replace('>', "\\>")
}

pub struct Generator {
    mods: Vec<ClientModDef>,
}

impl Generator {
    pub fn new(collection: &Collection) -> Self {
        Self {
            mods: Self::generate(&collection),
        }
    }

    fn generate(collection: &Collection) -> Vec<ClientModDef> {
        collection
            .iter()
            .map(|n| Self::generate_client(None, n))
            .collect()
    }

    fn generate_client(parent: Option<&Node>, node: &Node) -> ClientModDef {
        let path = node.value.path.iter().last().unwrap();
        Self::generate_client_impl(parent.is_some(), node, path)
    }

    fn generate_client_impl(has_parent: bool, node: &Node, segment: &PathElement) -> ClientModDef {
        let segment_no_braces = segment.as_string_without_braces();
        let segment_name = crate::name_to_underscore_name(&segment_no_braces);
        let mut client_name = crate::name_to_ident(&segment_name);
        client_name.push_str("Client");

        let client_name = Ident::new(&client_name, quote!().span());

        let mut enums = BTreeMap::new();

        let methods: Vec<_> = node
            .value
            .info
            .values()
            .filter_map(|info| {
                let method: String = info
                    .method
                    .chars()
                    .take(1)
                    .chain(info.method.chars().skip(1).map(|c| c.to_ascii_lowercase()))
                    .collect();

                let mut parameters = info
                    .parameters
                    .as_ref()
                    .map(|v| {
                        let mut type_def = v.type_def(&method, &node.value.path);

                        if let Some(def) = type_def.as_mut() {
                            def.hoist_enum_defs(&mut enums);
                        }

                        type_def
                    })
                    .flatten();

                let fn_name_str = method.to_ascii_lowercase();
                let fn_name = Ident::new(&fn_name_str, quote!().span());

                if let Some(parameters) = &mut parameters {
                    parameters.hoist_enum_defs(&mut enums);
                }

                let (signature, defer_signature, params_def) =
                    if let Some(TypeDef::Struct(strt)) = &parameters {
                        let name = Ident::new(strt.name(), quote! {}.span());
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

                    def.hoist_enum_defs(&mut enums);

                    let name = def.as_field_ty(ret.optional.get());

                    let call = match def.primitive() {
                        Some(PrimitiveTypeDef::Integer) => {
                            let int = proxmox_api(quote!(types::Integer));
                            quote!(Ok(self.client.#fn_name::<_, #int>(#defer_signature)?.get()))
                        }
                        Some(PrimitiveTypeDef::Number) => {
                            let num_ty = proxmox_api(quote!(types::Number));
                            quote!(Ok(self.client.#fn_name::<_, #num_ty>(#defer_signature)?.get()))
                        }
                        Some(PrimitiveTypeDef::Boolean) => {
                            let bool_ty = proxmox_api(quote!(types::Bool));
                            quote!(Ok(self.client.#fn_name::<_, #bool_ty>(#defer_signature)?.get()))
                        }
                        Some(PrimitiveTypeDef::String) | None => {
                            quote!(self.client.#fn_name(#defer_signature))
                        }
                    };

                    (quote! { -> Result<#name, T::Error> }, Some(def), call)
                } else {
                    (
                        quote!(),
                        None,
                        quote!(self.client.#fn_name(#defer_signature)),
                    )
                };

                let doc = if let Some(doc) = &info.description {
                    let doc = clean_doc(&doc);
                    let doc = Literal::string(&doc);
                    Some(quote! { #[doc = #doc] })
                } else {
                    None
                };

                let fn_definition = quote! {
                    #doc
                    pub fn #fn_name(#signature) #returns {
                        let path = self.path.to_string();
                        #call
                    }
                };

                let client = proxmox_api(quote!(client::Client));

                let block = quote! {
                    #params_def

                    #returns_definition

                    impl<T> #client_name<T> where T: #client {
                        #fn_definition
                    }
                };

                Some(block)
            })
            .collect();

        let mut child_defs = Vec::new();
        let child_constructors = node.children.iter().map(|(segment, child)| {
            assert_eq!(
                node.value.path.iter().count() + 1,
                child.value.path.iter().count(),
                "Child node is not a direct child of its parent, path-wise"
            );

            let segment_no_braces =
                crate::name_to_underscore_name(segment.as_string_without_braces());
            let mod_name = Ident::new(&segment_no_braces, quote!().span());

            let child_def = Self::generate_client(Some(node), child);

            let child_name = Ident::new(&child_def.client_name, quote!().span());
            let defer = quote! { #mod_name::#child_name::<T> };
            let child_fn_name = Ident::new(&segment_no_braces, quote!().span());

            let (new_sig, child_call) = match segment {
                PathElement::Literal(_) => (
                    quote! { #child_fn_name(&self) -> #defer },
                    quote! { #defer::new(self.client.clone(), &self.path) },
                ),
                PathElement::Placeholder(p) => {
                    let next_val_ty = if p == "vmid" {
                        proxmox_api(quote!(types::VmId))
                    } else {
                        quote!(&str)
                    };

                    let next_val_ident = Ident::new(p, quote!().span());
                    (
                        quote! { #child_fn_name(&self, #next_val_ident: #next_val_ty) -> #defer },
                        quote! { #defer::new(self.client.clone(), &self.path, #next_val_ident)},
                    )
                }
            };

            child_defs.push(child_def);

            let client = proxmox_api(quote!(client::Client));
            quote! {
                impl<T> #client_name<T> where T: #client {
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

        let client = proxmox_api(quote!(client::Client));
        let enums = enums.values();

        let definition = quote! {
            pub struct #client_name<T> {
                client: T,
                path: String,
            }

            impl<T> #client_name<T> where T: #client {
                #new
            }

            #(#enums)*

            #(#methods)*

            #(#child_constructors)*
        };

        ClientModDef {
            client_name: client_name.to_string(),
            name: segment_name.to_string(),
            client_tokens: definition,
            children: child_defs,
        }
    }

    fn make_placeholder_constructor(has_parent: bool, placeholder: &str) -> TokenStream {
        let placeholder_ident = Ident::new(placeholder, quote!().span());

        let placeholder_ty = if placeholder == "vmid" {
            proxmox_api(quote!(types::VmId))
        } else {
            quote!(&str)
        };

        if has_parent {
            quote! {
                pub fn new(client: T, parent_path: &str, #placeholder_ident: #placeholder_ty) -> Self {
                    Self {
                        client,
                        path: format!("{}/{}", parent_path, #placeholder_ident)
                    }
                }
            }
        } else {
            quote! {
                pub fn new(client: T, #placeholder_ident: #placeholder_ty) -> Self {
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

        if has_parent {
            quote! {
                pub fn new(client: T, parent_path: &str) -> Self {
                    Self {
                        client,
                        path: format!("{}{}", parent_path, #literal)
                    }
                }
            }
        } else {
            quote! {
                pub fn new(client: T) -> Self {
                    Self {
                        client,
                        path: #literal.to_string()
                    }
                }
            }
        }
    }

    fn generate_impl(has_parent: bool, def: &ClientModDef, stream: &mut TokenStream) {
        if !has_parent {
            stream.extend(def.client_tokens.clone());
            def.children
                .iter()
                .for_each(|c| Self::generate_impl(true, c, stream));
        } else {
            let mod_name = Ident::new(&def.name, quote!().span());

            stream.extend(quote!(pub mod #mod_name));
            Punct::new('{', proc_macro2::Spacing::Alone).to_tokens(stream);
            stream.extend(def.client_tokens.clone());
            def.children
                .iter()
                .for_each(|c| Self::generate_impl(true, c, stream));
            Punct::new('}', proc_macro2::Spacing::Alone).to_tokens(stream);
        }
    }
}

impl ToTokens for Generator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for def in &self.mods {
            Self::generate_impl(false, def, tokens);
        }
    }
}

impl IntoIterator for Generator {
    type Item = ClientModDef;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.mods.into_iter()
    }
}

impl core::ops::Deref for Generator {
    type Target = [ClientModDef];

    fn deref(&self) -> &Self::Target {
        &self.mods
    }
}
