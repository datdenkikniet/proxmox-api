use std::{borrow::Cow, collections::BTreeMap};

use serde::Deserialize;

use crate::{
    generator::{FieldDef, TypeDef},
    Path, PathElement,
};

use super::{ty::IntOrTy, Output, Type};

#[derive(Debug, Clone, Deserialize)]
pub struct Parameters<'a> {
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<Cow<'a, str>, Type<'a>>>,
    #[serde(default, skip_serializing_if = "IntOrTy::is_unset")]
    pub additional_properties: IntOrTy<'a>,
}

impl Parameters<'_> {
    pub fn type_def(&self, prefix: &str, path: &Path) -> Option<Output> {
        if let Some(properties) = &self.properties {
            let name = crate::name_to_ident(&format!("{prefix}Params"));

            let mut final_output = Output::new();
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

                let output = ty.type_def(name, "Returns")?;
                let typedef = output.def.clone()?;
                final_output.absorb(output);

                let doc = ty.doc();
                let optional = ty.optional.get();
                let (field_def, num_items_def) =
                    FieldDef::new(name.to_string(), typedef, optional, doc);

                final_output
                    .module_defs
                    .extend(num_items_def.map(|v| TypeDef::NumberedItems(Box::new(v))));

                Some(field_def)
            });

            let fields: Vec<_> = fields.collect();

            if fields.is_empty() {
                return None;
            }

            let (additional_properties, ext) = self
                .additional_properties
                .as_additional_properties("Params");
            final_output.module_defs.extend(ext);

            let type_def = TypeDef::new_struct(name.clone(), fields, additional_properties);
            final_output.def = Some(type_def);

            Some(final_output)
        } else {
            None
        }
    }
}
