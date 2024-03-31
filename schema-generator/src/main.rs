use std::path::PathBuf;

use proc_macro2::TokenStream;
use schema_generator::{
    raw::{
        flattened::{Collection, Node},
        TreeNode, Type,
    },
    Path, PathElement, StructDef,
};

use quote::quote;
use syn::{spanned::Spanned, Ident, Lit, LitStr};

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();

    args.next();

    let file_name = PathBuf::from(
        args.next()
            .expect("Expected first argument to be dumped JSON schema."),
    );

    let str = std::fs::read_to_string(file_name).unwrap();

    let tree: Vec<TreeNode> = serde_json::from_str(&str).unwrap();

    let collection = Collection::from_nodes(&tree).unwrap();

    let get = |name: &str| collection.path(&Path::try_from(name).unwrap()).unwrap();

    let node = get("/cluster");
    let node2 = get("/cluster/nextid");
    let node3 = get("/cluster/backup");
    let node4 = get("/cluster/backup/{vmid}");

    let node = Node {
        value: node.clone(),
        children: vec![
            (
                PathElement::Literal("nextid".to_string()),
                Node {
                    value: node2.clone(),
                    children: Vec::new(),
                },
            ),
            (
                PathElement::Literal("backup".to_string()),
                Node {
                    value: node3.clone(),
                    children: vec![(
                        PathElement::Placeholder("id".to_string()),
                        Node {
                            value: node4.clone(),
                            children: Vec::new(),
                        },
                    )],
                },
            ),
        ],
    };

    println!("{}", generate_client(None, &node).1);

    Ok(())
}

fn generate_client(parent: Option<&Node>, node: &Node) -> (String, TokenStream) {
    let path = node.value.path.iter().last().unwrap();

    match path {
        PathElement::Literal(literal) => generate_literal_client(parent.is_some(), node, &literal),
        PathElement::Placeholder(p) => generate_placeholder_client(parent.is_some(), node, &p),
    }
}

fn name_with_first_to_upper_case(name: &str) -> String {
    let mut chars = name.chars().map(|v| v.to_ascii_lowercase());
    let mut new_name = String::new();

    if let Some(char) = chars.next() {
        new_name.push(char.to_ascii_uppercase());
    }

    new_name.extend(chars);

    new_name
}

fn generate_placeholder_client(
    has_parent: bool,
    node: &Node,
    placeholder: &str,
) -> (String, TokenStream) {
    todo!()
}

fn generate_literal_client(has_parent: bool, node: &Node, segment: &str) -> (String, TokenStream) {
    let name = name_with_first_to_upper_case(segment);

    let name = Ident::new(&name, quote!().span());
    let segment_literal = Lit::Str(LitStr::new(segment, quote!().span()));

    let methods = node.value.info.values().map(|info| {
        let method = &info.method;

        let parameters = info
            .parameters
            .as_ref()
            .map(|v| {
                let prefix = name_with_first_to_upper_case(&method);
                v.struct_def(&prefix, &node.value.path)
            })
            .flatten();

        let fn_name_str = method.to_ascii_lowercase();
        let fn_name = Ident::new(&fn_name_str, quote!().span());

        let returns = info
            .returns
            .as_ref()
            .map(Type::to_tokens)
            .map(|v| quote! { -> Result<#v, ::proxmox_api::client::Error> })
            .unwrap_or(quote! {});

        let (fn_definition, params_def) = if let Some(StructDef { definition, name }) = parameters {
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

        quote! {
            #params_def

            impl #name {
                #fn_definition
            }
        }
    });

    let children = node.children.iter().map(|(segment, child)| {
        assert_eq!(
            node.value.path.iter().count() + 1,
            child.value.path.iter().count(),
            "Child node is not a direct child of its parent, path-wise"
        );

        let segment_no_braces = segment.as_string_without_braces();
        let mod_name = Ident::new(segment_no_braces, quote!().span());
        let (child_name, child_data) = generate_client(Some(node), child);
        let child_name = Ident::new(&child_name, quote!().span());
        let defer = quote! { #mod_name::#child_name };
        let child_fn_name = Ident::new(segment_no_braces, quote!().span());

        let new_sig = match segment {
            PathElement::Literal(_) => quote! { #child_fn_name(&self) -> #defer },
            PathElement::Placeholder(_) => quote! { #child_fn_name(&self, id: &str) -> #defer },
        };

        quote! {
            pub mod #mod_name {
                #child_data
            }

            impl #name {
                pub fn #new_sig {
                    #defer::new(self.client.clone(), &self.path)
                }
            }
        }
    });

    let new = if has_parent {
        quote! {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::client::Client>, parent_path: &str) -> Self {
                Self {
                    client,
                    path: format!("{}/{}", parent_path, #segment_literal)
                }
            }
        }
    } else {
        quote! {
            pub fn new(client: ::std::sync::Arc<::proxmox_api::client::Client>) -> Self {
                Self {
                    client,
                    path: #segment_literal.to_string()
                }
            }
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
