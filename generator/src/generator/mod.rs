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

mod mod_defs;
pub use mod_defs::ModuleDefs;

mod mod_def;
pub use self::mod_def::ClientModDef;

mod num_items_def;
pub(crate) use num_items_def::NumItemsDef;

mod type_def;
pub(crate) use type_def::{PrimitiveTypeDef, TypeDef};

pub fn proxmox_api(path: TokenStream) -> TokenStream {
    quote! { crate::#path }
}

pub fn proxmox_api_str(path: String) -> Literal {
    Literal::string(&format!("crate::{path}"))
}

pub fn clean_doc<T: AsRef<str>>(input: &T) -> impl Iterator<Item = String> + '_ {
    input
        .as_ref()
        .split('\n')
        .map(|v| v.trim().trim_start_matches('"').trim_end_matches('"'))
        .filter(|v| !v.is_empty())
        .flat_map(|v| [v, ""])
        .map(|v| {
            v.replace('<', "\\<")
                .replace('>', "\\>")
                .replace('[', "\\[")
                .replace(']', "\\]")
        })
}

pub struct Generator {
    mods: Vec<ClientModDef>,
}

impl Generator {
    pub fn new(collection: &Collection) -> Self {
        Self {
            mods: Self::generate(collection),
        }
    }

    fn generate(collection: &Collection) -> Vec<ClientModDef> {
        let mut global_defs = Vec::new();

        let output = collection
            .iter()
            .map(|n| Self::generate_client(None, n, &mut global_defs))
            .collect();

        output
    }

    fn generate_client(
        parent: Option<&Node>,
        node: &Node,
        global_defs: &mut Vec<TypeDef>,
    ) -> ClientModDef {
        let path = node.value.path.iter().last().unwrap();
        Self::generate_client_impl(parent.is_some(), node, path, global_defs)
    }

    fn generate_client_impl(
        has_parent: bool,
        node: &Node,
        segment: &PathElement,
        global_defs: &mut Vec<TypeDef>,
    ) -> ClientModDef {
        let segment_no_braces = segment.as_string_without_braces();
        let segment_name = crate::name_to_underscore_name(segment_no_braces);
        let mut client_name = crate::name_to_ident(&segment_name);
        client_name.push_str("Client");

        let client_name = Ident::new(&client_name, quote!().span());

        let mut module_defs = ModuleDefs::default();

        let methods: Vec<_> = node
            .value
            .info
            .values()
            .map(|info| {
                let method: String = info
                    .method
                    .chars()
                    .take(1)
                    .chain(info.method.chars().skip(1).map(|c| c.to_ascii_lowercase()))
                    .collect();

                let parameters = info
                    .parameters
                    .as_ref()
                    .and_then(|v| v.type_def(&method, &node.value.path));

                let parameters = if let Some(output) = parameters {
                    module_defs.extend(output.def.clone());
                    module_defs.extend(output.module_defs);
                    global_defs.extend(output.global_defs);
                    output.def
                } else {
                    None
                };

                let fn_name_str = method.to_ascii_lowercase();
                let fn_name = Ident::new(&fn_name_str, quote!().span());

                let (signature, defer_signature) = if let Some(TypeDef::Struct(strt)) = &parameters
                {
                    let name = Ident::new(&strt.name(), quote! {}.span());
                    (quote!(&self, params: #name), quote!(&path, &params))
                } else {
                    (quote!(&self), quote!(&path, &()))
                };

                let (returns, call) = if let Some(ret) = info.returns.as_ref() {
                    let output = ret.type_def("", &format!("{method}Output"));

                    if let Some(output) = output {
                        let def = output.def.unwrap().clone();
                        module_defs.push(def.clone());
                        module_defs.extend(output.module_defs);
                        global_defs.extend(output.global_defs);

                        let (_, name) = def.as_field_ty(ret.optional.get());

                        let call = Self::to_call(def.primitive(), &fn_name, defer_signature);

                        (quote! { -> Result<#name, T::Error> }, call)
                    } else {
                        (
                            quote!( -> Result<(), T::Error> ),
                            quote!(self.client.#fn_name(#defer_signature)),
                        )
                    }
                } else {
                    (quote!(), quote!(self.client.#fn_name(#defer_signature)))
                };

                let doc = if let Some(doc) = &info.description {
                    let doc = clean_doc(&doc);
                    let doc = doc.map(|doc| Literal::string(&doc));
                    Some(quote! { #(#[doc = #doc])* })
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
                    impl<T> #client_name<T> where T: #client {
                        #fn_definition
                    }
                };

                block
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

            let child_def = Self::generate_client(Some(node), child, global_defs);

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

        let definition = quote! {
            #[derive(Debug, Clone)]
            pub struct #client_name<T> {
                client: T,
                path: String,
            }

            impl<T> #client_name<T> where T: #client {
                #new
            }

            #(#methods)*

            #module_defs

            #(#child_constructors)*
        };

        ClientModDef {
            client_name: client_name.to_string(),
            name: segment_name.to_string(),
            client_tokens: definition,
            children: child_defs,
        }
    }

    // This is a separate method because cargo fmt breaks if the tokens in this
    // function are too far to the right, for some reason...
    fn to_call(
        def: Option<PrimitiveTypeDef>,
        fn_name: &Ident,
        defer_signature: TokenStream,
    ) -> TokenStream {
        match def {
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
