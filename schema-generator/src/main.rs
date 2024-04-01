use std::path::PathBuf;

use proc_macro2::TokenStream;
use schema_generator::{
    raw::{flattened::Collection, TreeNode},
    Generator,
};

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
    let generator = Generator::new(collection);

    let mut tokens = TokenStream::new();
    generator.generate(&mut tokens);
    println!("{}", tokens);

    Ok(())
}
