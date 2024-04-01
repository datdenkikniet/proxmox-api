use std::{borrow::Cow, collections::HashMap};

use serde::Deserialize;

use crate::{
    generator::{FieldDef, TypeDef},
    Path, PathElement,
};

use super::Type;

#[derive(Debug, Clone, Deserialize)]
pub struct Parameters<'a> {
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<Cow<'a, str>, Type<'a>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}

impl Parameters<'_> {
    pub fn type_def(&self, prefix: &str, path: &Path) -> Option<TypeDef> {
        if let Some(properties) = &self.properties {
            let name = crate::name_to_ident(&format!("{prefix}Params"));

            let mut external_defs = Vec::new();
            let fields = properties.iter().filter_map(|(name, ty)| {
                if path.iter().any(|p| {
                    if let PathElement::Placeholder(placeholder) = p {
                        placeholder == name
                    } else {
                        false
                    }
                }) {
                    return None;
                }

                let typedef = ty.type_def(name, "Returns");
                external_defs.push(typedef.clone());

                let doc = ty.doc();
                let optional = ty.optional.get();
                Some(FieldDef::new(name.to_string(), typedef, optional, doc))
            });

            let fields: Vec<_> = fields.collect();

            if fields.is_empty() {
                return None;
            }

            let type_def = TypeDef::new_struct(name.clone(), fields, external_defs);

            Some(type_def)
        } else {
            None
        }
    }
}
