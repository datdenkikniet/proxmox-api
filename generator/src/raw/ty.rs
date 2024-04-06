use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
};

use serde::{Deserialize, Serialize};

use crate::generator::{AdditionalProperties, FieldDef, PrimitiveTypeDef, TypeDef};

use super::{Format, KnownFormat, Optional};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Type<'a> {
    #[serde(flatten, borrow, default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<TypeKind<'a>>,

    #[serde(
        rename = "typetext",
        borrow,
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_text: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Optional::is_empty")]
    pub optional: Optional,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verbose_description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Format<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format_description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_key: Option<u32>,
    // Another field in the parent object is required
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renderer: Option<Cow<'a, str>>,
    // This type is an alias for a field in the parent object
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Cow<'a, str>>,
    // This type is an alias for a field in the parent object.
    #[serde(
        alias = "keyAlias",
        borrow,
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_alias: Option<Cow<'a, str>>,
}

impl Type<'_> {
    pub fn doc(&self) -> impl Iterator<Item = String> + '_ {
        self.description
            .as_ref()
            .into_iter()
            .chain(self.verbose_description.as_ref().into_iter())
            .map(Cow::to_string)
    }

    pub fn type_def(&self, field_name: &str, struct_suffix: &str) -> TypeDef {
        let output_type = if let Some(ty) = self.ty.as_ref() {
            match ty {
                TypeKind::Null => TypeDef::Unit,
                TypeKind::String {
                    enum_values,
                    default,
                    ..
                } => {
                    if let Some(enum_values) = enum_values {
                        let default = if enum_values.iter().any(|v| Some(v) == default.as_ref()) {
                            default.as_ref().map(Cow::to_string)
                        } else {
                            None
                        };

                        let enum_values: BTreeSet<_> = enum_values
                            .iter()
                            .map(Cow::to_string)
                            .chain(default.clone())
                            .collect();
                        let no_derives = Option::<&str>::None;

                        // TODO: Fix this ugly hack...
                        // Would be nice to find a better workaround that doesn't require one-offs
                        // and also doesn't require global state...
                        let name = if enum_values.contains("cdrom-image-ignored") {
                            crate::name_to_ident("warning_type")
                        } else {
                            crate::name_to_ident(field_name)
                        };

                        TypeDef::new_enum(name, no_derives, enum_values, default)
                    } else {
                        TypeDef::Primitive(PrimitiveTypeDef::String)
                    }
                }
                TypeKind::Number { .. } => TypeDef::Primitive(PrimitiveTypeDef::Number),
                TypeKind::Integer { .. } => TypeDef::Primitive(PrimitiveTypeDef::Integer),
                TypeKind::Boolean { .. } => TypeDef::Primitive(PrimitiveTypeDef::Boolean),
                TypeKind::Array { items } => {
                    let inner = items.type_def(field_name, &format!("{struct_suffix}Items"));

                    TypeDef::Array {
                        inner: Box::new(inner),
                    }
                }
                TypeKind::Object {
                    properties,
                    additional_properties,
                } => {
                    let field_name = crate::name_to_ident(field_name);
                    let suffix = crate::name_to_ident(struct_suffix);
                    let struct_name = format!("{field_name}{suffix}");

                    let additional_props =
                        additional_properties.as_additional_properties(struct_suffix);

                    if let Some(props) = properties {
                        let mut external_defs: Vec<TypeDef> = Vec::new();

                        let fields: Vec<_> = props
                            .iter()
                            .filter_map(|(original_name, ty)| {
                                let field_name = crate::name_to_ident(&original_name);
                                let inner = ty
                                    .type_def(&field_name, &format!("{struct_suffix}{field_name}"));

                                external_defs.push(inner.clone());

                                let doc = ty.doc();
                                let optional = ty.optional.get();

                                if inner.is_unit() && !optional {
                                    None
                                } else {
                                    Some(FieldDef::new(
                                        original_name.to_string(),
                                        inner,
                                        optional,
                                        doc,
                                    ))
                                }
                            })
                            .collect();

                        TypeDef::new_struct(struct_name, fields, additional_props, external_defs)
                    } else if !additional_props.is_none() {
                        TypeDef::new_struct(struct_name, Vec::new(), additional_props, Vec::new())
                    } else {
                        TypeDef::Unit
                    }
                }
            }
        } else {
            TypeDef::Unit
        };

        if let (Some(fallback), Some(Format::Kind(format))) =
            (output_type.primitive(), &self.format)
        {
            let format = match format {
                KnownFormat::MacAddr { .. } => {
                    let allow_ig_bit = self
                        .verbose_description
                        .as_ref()
                        .map(|v| !v.contains("the I/G (Individual/Group) bit not set"))
                        .unwrap_or(true);
                    KnownFormat::MacAddr(allow_ig_bit)
                }
                format => *format,
            };

            TypeDef::KnownType { format, fallback }
        } else {
            output_type
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum IntOrTy<'a> {
    Int(u32),
    #[serde(borrow)]
    Ty(Box<Type<'a>>),
}

impl IntOrTy<'_> {
    pub fn as_additional_properties(&self, struct_suffix: &str) -> AdditionalProperties {
        match self {
            IntOrTy::Int(1) => AdditionalProperties::Untyped,
            IntOrTy::Ty(ty) => {
                let ty = ty.type_def("additional_properties", struct_suffix);
                AdditionalProperties::Type(Box::new(ty))
            }
            _ => AdditionalProperties::None,
        }
    }

    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Int(0))
    }
}

impl Default for IntOrTy<'_> {
    fn default() -> Self {
        Self::Int(1)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum TypeKind<'a> {
    Null,
    String {
        #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
        max_length: Option<u32>,
        #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
        min_length: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pattern: Option<Cow<'a, str>>,

        // If it's an enum
        #[serde(rename = "enum", default, skip_serializing_if = "Option::is_none")]
        enum_values: Option<Vec<Cow<'a, str>>>,
        #[serde(default)]
        default: Option<Cow<'a, str>>,
    },
    Number {
        #[serde(default, alias = "min", skip_serializing_if = "Option::is_none")]
        minimum: Option<serde_json::Value>,
        #[serde(default, alias = "max", skip_serializing_if = "Option::is_none")]
        maximum: Option<f32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<f32>,
    },
    Integer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<u32>,
    },
    Boolean {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        default: Option<u32>,
    },
    Array {
        items: Box<Type<'a>>,
    },
    Object {
        #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
        properties: Option<BTreeMap<Cow<'a, str>, Type<'a>>>,
        #[serde(
            default,
            skip_serializing_if = "IntOrTy::is_unset",
            rename = "additionalProperties"
        )]
        additional_properties: IntOrTy<'a>,
    },
}
