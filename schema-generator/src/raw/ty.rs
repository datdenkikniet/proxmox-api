use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::generator::{FieldDef, PrimitiveTypeDef, TypeDef};

use super::{Format, Optional};

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Optional::is_empty")]
    pub optional: Optional,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verbose_description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Box<Format<'a>>>,
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum IntOrTy<'a> {
    Int(u32),
    #[serde(borrow)]
    Ty(Type<'a>),
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
        maximum: Option<u32>,
    },
    Integer {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        minimum: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        maximum: Option<u32>,
    },
    Boolean,
    Array {
        items: Box<Type<'a>>,
    },
    Object {
        #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
        properties: Option<HashMap<Cow<'a, str>, Type<'a>>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            rename = "additionalProperties"
        )]
        additional_properties: Option<Box<IntOrTy<'a>>>,
    },
}

impl Type<'_> {
    pub fn type_def(
        &self,
        field_name: &str,
        struct_prefix: &str,
        struct_suffix: &str,
        enum_prefix: &str,
    ) -> TypeDef {
        if let Some(ty) = self.ty.as_ref() {
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

                        let enum_values = enum_values
                            .iter()
                            .map(Cow::to_string)
                            .chain(default.clone())
                            .collect();
                        let no_derives = Option::<&str>::None;

                        let name = crate::name_to_ident(&format!("{enum_prefix}{field_name}"));
                        TypeDef::new_enum(name, no_derives, enum_values, default)
                    } else {
                        TypeDef::Primitive(PrimitiveTypeDef::String)
                    }
                }
                TypeKind::Number { .. } => TypeDef::Primitive(PrimitiveTypeDef::Number),
                TypeKind::Integer { .. } => TypeDef::Primitive(PrimitiveTypeDef::Integer),
                TypeKind::Boolean => TypeDef::Primitive(PrimitiveTypeDef::Boolean),
                TypeKind::Array { items } => {
                    let inner = items.type_def(
                        field_name,
                        struct_prefix,
                        &format!("{struct_suffix}Items"),
                        &enum_prefix,
                    );

                    TypeDef::Array {
                        inner: Box::new(inner),
                    }
                }
                TypeKind::Object {
                    properties,
                    additional_properties,
                } => {
                    if let Some(IntOrTy::Ty(additional_props)) = additional_properties.as_deref() {
                        assert!(properties.is_none(), "Cannot handle combination of typed additional properties & normal properties.");

                        additional_props.type_def(
                            field_name,
                            struct_prefix,
                            struct_suffix,
                            enum_prefix,
                        )
                    } else if let Some(props) = properties {
                        let mut external_defs: Vec<TypeDef> = Vec::new();

                        let fields: Vec<_> = props
                            .iter()
                            .map(|(original_name, ty)| {
                                let field_name = crate::name_to_underscore_name(&original_name);
                                let rename = if &field_name != original_name {
                                    Some(original_name.to_string())
                                } else {
                                    None
                                };

                                let field_name = crate::name_to_ident(&original_name);
                                let inner = ty.type_def(
                                    &field_name,
                                    struct_prefix,
                                    struct_suffix,
                                    enum_prefix,
                                );

                                external_defs.push(inner.clone());

                                FieldDef::new(rename, field_name, inner, ty.optional.get())
                            })
                            .collect();

                        let dervs = if fields.iter().all(|f| f.optional()) {
                            &["Default"][..]
                        } else {
                            &[][..]
                        };

                        let struct_name = crate::name_to_ident(&format!(
                            "{struct_prefix}{field_name}{struct_suffix}"
                        ));
                        TypeDef::new_struct(struct_name, dervs, fields, external_defs)
                    } else {
                        TypeDef::Unit
                    }
                }
            }
        } else {
            TypeDef::Unit
        }
    }
}
