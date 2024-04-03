use schema_generator::{
    raw::{flattened::Collection, TreeNode},
    Generator,
};

use clap::Parser;

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
        Cli::Recursive { file, .. } => generator.generate_file_tree(file),
        Cli::NonRecursive { file, .. } => generator.generate_single_file(file),
    }
}
