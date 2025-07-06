pub mod raw;

mod path;
pub use path::{Path, PathElement};

mod generator;
pub use generator::{ClientModDef, Generator};

use clap::Parser;
use raw::{flattened::Collection, TreeNode};

mod parser;
pub use parser::api;

const RENAME_MAP: &[(&str, &str)] = &[
    ("type", "ty"),
    ("macro", "macro_def"),
    ("in", "in_name"),
    ("ref", "reference"),
];

fn check_rename(name: String) -> String {
    if let Some(to) = RENAME_MAP
        .iter()
        .find_map(|(from, to)| if from == &name { Some(to) } else { None })
    {
        to.to_string()
    } else {
        name
    }
}

pub(crate) fn name_to_underscore_name(name: &str) -> String {
    let name: String = name
        .chars()
        .map(|v| v.to_ascii_lowercase())
        .filter_map(|v| {
            if v == '-' || v == '.' || v == '[' || v == '{' {
                Some('_')
            } else if v == ']' || v == '}' {
                None
            } else {
                Some(v)
            }
        })
        .collect();

    check_rename(name)
}

pub(crate) fn name_to_ident(name: &str) -> String {
    let mut chars = name.chars();
    let mut new_name = String::new();

    for char in chars.by_ref() {
        if char == '.'
            || char == '{'
            || char == '}'
            || char == '['
            || char == ']'
            || char == '_'
            || char == '-'
        {
            continue;
        } else {
            new_name.push(char.to_ascii_uppercase());
            break;
        }
    }

    if let Some(v) = new_name.chars().next() {
        if v.is_numeric() {
            new_name = format!("_{new_name}");
        }
    }

    let mut prev_was_dash = false;
    for char in chars {
        if char == '}' || char == ']' {
            continue;
        } else if char == '-'
            || char == '_'
            || char == '{'
            || char == '['
            || char == '+'
            || char == '.'
        {
            prev_was_dash = true;
            continue;
        } else {
            if prev_was_dash {
                new_name.push(char.to_ascii_uppercase());
            } else {
                new_name.push(char);
            }

            prev_was_dash = false;
        }
    }

    check_rename(new_name)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Recursive,
    NonRecursive,
}

#[derive(Parser)]
pub enum Cli {
    Recursive { file: String },
    NonRecursive { file: String },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let str = {
        let text = reqwest::blocking::get("https://pve.proxmox.com/pve-docs/api-viewer/apidoc.js")
            .unwrap()
            .text()
            .unwrap();

        api::extract_api_schema(&text).expect("failed to get schema")
    };

    let tree: Vec<TreeNode> = serde_json::from_str(&str).unwrap();

    let collection = Collection::from_nodes(&tree);
    let generator = Generator::new(&collection);

    match cli {
        Cli::Recursive { file, .. } => generator.generate_file_tree(file),
        Cli::NonRecursive { file, .. } => generator.generate_single_file(file),
    }
}
