use std::path::Path;
use std::{io::Write, path::PathBuf};

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
    let generator = Generator::new(&collection);

    match cli {
        Cli::Recursive { file, .. } => generate_tree(file, generator),
        Cli::NonRecursive { file, .. } => generate(file, generator),
    }
}

fn open_rw_truncate<P>(path: P) -> std::io::Result<std::fs::File>
where
    P: AsRef<Path>,
{
    std::fs::File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .append(false)
        .open(&path)
}

fn generate<P>(output: P, generator: Generator) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = open_rw_truncate(&output)?;
    write!(file, "{}", generator.to_token_stream())
}

fn generate_tree<P>(output: P, generator: Generator) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = open_rw_truncate(&output)?;

    let child_names = generator
        .iter()
        .map(|c| Ident::new(&c.name, quote!().span()));

    let child_mods = quote! { #(mod #child_names;)* };
    write!(file, "{child_mods}")?;

    let mut buf = output.as_ref().to_path_buf();
    buf.set_extension("");

    for def in generator {
        generate_tree_impl(&buf, def)?;
    }

    Ok(())
}

fn generate_tree_impl(path: &PathBuf, def: ClientModDef) -> std::io::Result<()> {
    std::fs::create_dir_all(path)?;

    let my_dir = path.join(def.name);

    let my_file = my_dir.with_extension("rs");
    println!("Writing new file {:?}", my_file);
    let mut my_file = open_rw_truncate(my_file)?;

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
            generate_tree_impl(&my_dir, def)?;
        }
    }

    Ok(())
}
