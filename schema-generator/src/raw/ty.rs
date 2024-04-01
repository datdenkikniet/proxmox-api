use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use quote::quote;
use serde::{Deserialize, Serialize};
use syn::{spanned::Spanned, Ident};

use crate::generator::def::{FieldDef, PrimitiveTypeDef, TypeDef};

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
        enum_values: Option<HashSet<Cow<'a, str>>>,
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
    pub fn to_definition(&self, struct_name: &str) -> TypeDef {
        let struct_name = crate::name_to_struct_name(struct_name);

        if let Some(ty) = self.ty.as_ref() {
            match ty {
                TypeKind::Null => TypeDef::Unit,
                TypeKind::String {
                    enum_values,
                    default,
                    ..
                } => TypeDef::Primitive(PrimitiveTypeDef::String),
                TypeKind::Number { .. } => TypeDef::Primitive(PrimitiveTypeDef::Number),
                TypeKind::Integer { .. } => TypeDef::Primitive(PrimitiveTypeDef::Integer),
                TypeKind::Boolean => TypeDef::Primitive(PrimitiveTypeDef::Boolean),
                TypeKind::Array { items } => {
                    let inner = items.to_definition(&format!("{struct_name}Items"));

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

                        additional_props.to_definition(&struct_name)
                    } else if let Some(props) = properties {
                        let mut external_defs = Vec::new();
                        let fields: Vec<_> = props
                            .iter()
                            .map(|(original_name, ty)| {
                                let field_name = crate::name_to_underscore_name(&original_name);
                                let rename = if &field_name != original_name {
                                    Some(original_name.to_string())
                                } else {
                                    None
                                };

                                let optional = ty.optional.get();
                                let inner = ty.to_definition(&format!(
                                    "{struct_name}{}",
                                    crate::name_to_struct_name(&original_name)
                                ));

                                let ty = inner.as_field_ty(ty.optional.get());
                                let primitive_ty = inner.primitive();
                                external_defs.push(inner);

                                FieldDef {
                                    rename,
                                    name: field_name,
                                    ty,
                                    optional,
                                    primitive_ty,
                                }
                            })
                            .collect();

                        let name = Ident::new(&struct_name, quote!().span());

                        let dervs = if fields.iter().all(|f| f.optional) {
                            &["Default"][..]
                        } else {
                            &[][..]
                        };

                        TypeDef::new_struct(quote!(#name), dervs, fields, external_defs)
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
