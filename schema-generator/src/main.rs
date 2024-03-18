use std::path::PathBuf;

use schema_generator::{
    parsed::Parser,
    raw::{flattened::Collection, TreeNode},
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

    let flattened = Collection::from_nodes(&tree).unwrap();
    println!("Total of {} nodes.", flattened.len());

    Parser::parse(&flattened);

    Ok(())
}
