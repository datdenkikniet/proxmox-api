use crate::Path;

use super::{
    flattened::{Collection, Value},
    Format, FormatProperty, Items, Property,
};

#[derive(Debug)]
pub struct SchemaWarning {
    node_path: Path,
    node_element: String,
    variant: SchemaWarningVariant,
}

impl core::fmt::Display for SchemaWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SchemaWarningVariant::*;

        let path = format!("{}.{}", self.node_path, self.node_element);

        match &self.variant {
            NumberIsStringParseableAsNumber(ty, s) => write!(f, "default value {path} is number type '{ty}', but its value is a string parseable as number. Value: '{s}'."),
            PropertyOptional(s) => write!(f, "PO: property {path} is non-bool value: '{s}'"),
            PropertyMinimum(ty, s) => write!(f, "minimum value {path} is number type {ty}, but its value is a string parseable as a number. Value: '{s}'"),
            FormatPropertyMinimum(s) => write!(f, "FPM: format property {path} has non-number value, but should be u32. Value: '{s}'"),
            FormatPropertyDefaultIsU32(s) => write!(f, "FPM: format property {path} has a type 'string', but it is parsable as u32. Value: '{s}'"),
        }
    }
}

impl SchemaWarning {
    pub fn new(node_path: Path, node_element: String, variant: SchemaWarningVariant) -> Self {
        Self {
            node_path,
            node_element,
            variant,
        }
    }
}

#[derive(Debug)]
pub enum SchemaWarningVariant {
    NumberIsStringParseableAsNumber(String, String),
    PropertyOptional(serde_json::Value),
    PropertyMinimum(String, String),
    FormatPropertyDefaultIsU32(String),
    FormatPropertyMinimum(serde_json::Value),
}

pub struct SchemaValidator;

type List<'a> = &'a mut Vec<SchemaWarning>;

impl SchemaValidator {
    pub fn validate(schema: &Collection) -> Vec<SchemaWarning> {
        let mut warnings = Vec::new();

        schema
            .iter()
            .for_each(|v| Self::validate_value(&mut warnings, v));

        warnings
    }

    fn validate_value(warnings: List, value: &Value) {
        for (k, v) in value.info.iter() {
            let node_path = &value.path;

            let params_path = format!("{k}.parameters.properties");
            for (prop_key, prop) in v.parameters.properties.iter() {
                let path = format!("{params_path}.{prop_key}");
                Self::validate_property(node_path, warnings, &path, prop);
            }

            let returns_props_params = format!("{k}.returns.properties");
            for (prop_key, prop) in v.returns.iter().map(|r| r.properties.iter()).flatten() {
                let path = format!("{returns_props_params}.{prop_key}");
                Self::validate_property(node_path, warnings, &path, prop);
            }
        }
    }

    fn validate_property(node: &Path, warnings: List, path: &str, prop: &Property) {
        use SchemaWarningVariant::*;

        let mut warn = |field: &str, var| {
            let warning = SchemaWarning::new(node.clone(), format!("{}.{field}", path), var);
            warnings.push(warning);
        };

        if let (Some(ty), Some(serde_json::Value::String(s))) = (&prop.ty, &prop.default) {
            if (ty == "number" || ty == "integer" || ty == "boolean") && s.parse::<u32>().is_ok() {
                warn(
                    "default",
                    NumberIsStringParseableAsNumber(ty.to_string(), s.clone()),
                );
            }
        }

        if let Some(optional) = &prop.optional {
            if !matches!(optional, serde_json::Value::Number(_)) {
                warn("optional", PropertyOptional(optional.clone()));
            }
        }

        if let (Some(ty), Some(serde_json::Value::String(s))) = (&prop.ty, &prop.minimum) {
            if (ty == "number" || ty == "integer" || ty == "boolean") && s.parse::<u32>().is_ok() {
                warn("default", PropertyMinimum(ty.to_string(), s.clone()));
            }
        }

        if let Some(items) = &prop.items {
            Self::validate_items(node, warnings, path, items);
        }

        if let Some(format) = &prop.format {
            Self::validate_format(node, warnings, path, format);
        }
    }

    fn validate_items(node: &Path, warnings: &mut Vec<SchemaWarning>, path: &str, items: &Items) {
        let path = format!("{path}.items");

        if let Some(format) = &items.format {
            Self::validate_format(node, warnings, &path, &format);
        }

        if let Some(items) = &items.items {
            Self::validate_items(node, warnings, &path, items);
        }
    }

    fn validate_format(node: &Path, warnings: List, path: &str, format: &Format) {
        let path = format!("{path}.format");

        if let Format::Properties(props) = format {
            props.iter().for_each(|(k, prop)| {
                let path = format!("{path}.{k}");
                Self::validate_format_property(node, warnings, &path, prop);
            });
        }
    }

    fn validate_format_property(node: &Path, warnings: List, path: &str, prop: &FormatProperty) {
        use SchemaWarningVariant::*;

        let mut warn = |element, variant| {
            let warning = SchemaWarning::new(node.clone(), format!("{path}.{element}"), variant);
            warnings.push(warning);
        };

        match &prop.minimum {
            Some(serde_json::Value::Number(_)) => {}
            Some(v) => warn("minimum", FormatPropertyMinimum(v.clone())),
            _ => {}
        }

        match &prop.default {
            Some(serde_json::Value::String(s)) => {
                if s.parse::<u32>().is_ok() {
                    warn("default", FormatPropertyDefaultIsU32(s.clone()));
                }
            }
            _ => {}
        }

        if let Some(format) = &prop.format {
            Self::validate_format(node, warnings, &path, format);
        }
    }
}
