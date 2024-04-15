use std::io::BufReader;

use generator::{
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

    let str = if input_path.ends_with(".xz") {
        let file = std::fs::OpenOptions::new()
            .read(true)
            .open(input_path)
            .unwrap();

        let mut reader = BufReader::new(file);

        let mut output = Vec::new();
        lzma_rs::xz_decompress(&mut reader, &mut output).unwrap();

        String::from_utf8(output).unwrap()
    } else {
        std::fs::read_to_string(input_path).unwrap()
    };

    let tree: Vec<TreeNode> = serde_json::from_str(&str).unwrap();

    let collection = Collection::from_nodes(&tree);
    let generator = Generator::new(&collection);

    match cli {
        Cli::Recursive { file, .. } => generator.generate_file_tree(file),
        Cli::NonRecursive { file, .. } => generator.generate_single_file(file),
    }
}
