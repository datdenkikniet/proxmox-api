use crate::raw::flattened::{Collection, Node};
use crate::raw::{ALL_KNOWN_FORMATS, Format, IntOrTy, KnownFormat, Type, TypeKind};
use std::collections::HashSet;

pub struct FormatTracker {
    used_formats: HashSet<KnownFormat>,
}

impl FormatTracker {
    pub fn new() -> Self {
        Self {
            used_formats: HashSet::new(),
        }
    }

    pub fn analyze_known_formats(&mut self, collection: &Collection) {
        self.used_formats.clear();

        for node in collection.iter() {
            self.analyze_node(node);
        }
    }

    pub fn print_unused(&self) {
        for format in ALL_KNOWN_FORMATS.difference(&self.used_formats) {
            log::warn!("unused format: {}", format_name(format));
        }
    }

    fn analyze_node(&mut self, node: &Node) {
        for info in node.value.info.values() {
            if let Some(parameters) = &info.parameters {
                if let Some(properties) = &parameters.properties {
                    for ty in properties.values() {
                        self.analyze_type(ty);
                    }
                }

                if let IntOrTy::Ty(ty) = &parameters.additional_properties {
                    self.analyze_type(ty);
                }
            }

            if let Some(returns) = &info.returns {
                self.analyze_type(returns);
            }
        }

        for (_, child) in &node.children {
            self.analyze_node(child);
        }
    }

    fn analyze_type(&mut self, ty: &Type) {
        if let Some(format) = &ty.format {
            self.analyze_format(format);
        }

        if let Some(kind) = &ty.ty {
            match kind {
                TypeKind::Array { items } => self.analyze_type(items),
                TypeKind::Object {
                    properties,
                    additional_properties,
                } => {
                    if let Some(properties) = properties {
                        for ty in properties.values() {
                            self.analyze_type(ty);
                        }
                    }

                    if let IntOrTy::Ty(ty) = additional_properties {
                        self.analyze_type(ty);
                    }
                }
                _ => {}
            }
        }
    }

    fn analyze_format(&mut self, format: &Format) {
        match format {
            Format::Kind(known) => self.record_used(known),
            Format::Properties(properties) => {
                for ty in properties.values() {
                    self.analyze_type(ty);
                }
            }
        }
    }

    fn record_used(&mut self, format: &KnownFormat) {
        let normalized = match format {
            KnownFormat::Unknown(_) => return,
            other => other.clone(),
        };
        self.used_formats.insert(normalized);
    }
}

fn format_name(format: &KnownFormat) -> String {
    match format {
        KnownFormat::Unknown(_) => panic!("Unknown format should never be tracked"),
        _ => format.to_string(),
    }
}
