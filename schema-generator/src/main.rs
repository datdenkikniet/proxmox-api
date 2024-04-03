use std::path::Path;
use std::{io::Write, path::PathBuf};

use proc_macro2::{Punct, TokenStream};
use schema_generator::{
    raw::{flattened::Collection, TreeNode},
    ClientModDef, Generator,
};

use clap::Parser;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, Ident};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Recursive,
    NonRecursive,
}

#[derive(Parser)]
pub enum Cli {
    Recursive { input_path: String, file: String },
    NonRecursive { input_path: String, file: String },
}

impl Cli {
    pub fn input_path(&self) -> &str {
        match self {
            Cli::Recursive { input_path, .. } => input_path,
            Cli::NonRecursive { input_path, .. } => input_path,
        }
    }
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let input_path = cli.input_path();
    let str = std::fs::read_to_string(input_path).unwrap();

    let tree: Vec<TreeNode> = serde_json::from_str(&str).unwrap();

    let collection = Collection::from_nodes(&tree).unwrap();
    let generator = Generator::new(collection);
    let generated = generator.generate();

    match cli {
        Cli::Recursive { file, .. } => generate_file(file, generated),
        Cli::NonRecursive { file, .. } => generate(file, generated),
    }
}

fn open_rw_append<P>(path: P) -> std::io::Result<std::fs::File>
where
    P: AsRef<Path>,
{
    std::fs::File::options()
        .write(true)
        .create(true)
        .append(false)
        .open(&path)
}

fn generate_file<P>(output: P, defs: Vec<ClientModDef>) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = open_rw_append(&output)?;

    let child_names = defs.iter().map(|c| Ident::new(&c.name, quote!().span()));

    let child_mods = quote! { #(mod #child_names;)* };
    write!(file, "{child_mods}")?;

    let mut buf = output.as_ref().to_path_buf();
    buf.set_extension("");

    for def in defs {
        generate_file_impl(&buf, def)?;
    }

    Ok(())
}

fn generate_file_impl(path: &PathBuf, def: ClientModDef) -> std::io::Result<()> {
    std::fs::create_dir_all(path)?;

    let my_dir = path.join(def.name);

    let my_file = my_dir.with_extension("rs");
    println!("Writing new file {:?}", my_file);
    let mut my_file = open_rw_append(my_file)?;

    let child_names = def
        .children
        .iter()
        .map(|c| Ident::new(&c.name, quote!().span()));

    let child_mods = quote! {#(mod #child_names;)* };
    let body = &def.client_tokens;

    write!(my_file, "{child_mods}{body}")?;

    if !def.children.is_empty() {
        std::fs::create_dir_all(&my_dir)?;
        for def in def.children {
            generate_file_impl(&my_dir, def)?;
        }
    }

    Ok(())
}

fn generate<P>(output: P, defs: Vec<ClientModDef>) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = std::fs::File::options()
        .write(true)
        .create(true)
        .append(false)
        .open(output)?;

    let mut tokens = TokenStream::new();
    for def in defs {
        generate_impl(false, def, &mut tokens);
    }

    write!(file, "{tokens}")
}

fn generate_impl(has_parent: bool, def: ClientModDef, stream: &mut TokenStream) {
    if !has_parent {
        stream.extend(def.client_tokens);
        def.children
            .into_iter()
            .for_each(|c| generate_impl(true, c, stream));
    } else {
        let mod_name = Ident::new(&def.name, quote!().span());

        stream.extend(quote!(pub mod #mod_name));
        Punct::new('{', proc_macro2::Spacing::Alone).to_tokens(stream);
        stream.extend(def.client_tokens);
        def.children
            .into_iter()
            .for_each(|c| generate_impl(true, c, stream));
        Punct::new('}', proc_macro2::Spacing::Alone).to_tokens(stream);
    }
}
