use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
};

use serde::Deserialize;

use super::{Format, Items, Optional, Parameters};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Property<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
    // Can be string if type is 'enum'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Optional::is_empty")]
    pub optional: Optional,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(rename = "type", borrow)]
    pub ty: Option<Cow<'a, str>>,
    #[serde(rename = "typetext", borrow, default)]
    pub ty_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub format: Option<Format<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format_description: Option<Cow<'a, str>>,
    pub pattern: Option<String>,
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    // Basically always u64, but not aalway :/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u32>,
    #[serde(borrow, rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<HashSet<Cow<'a, str>>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Items<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub verbose_description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub renderer: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<Cow<'a, str>, Property<'a>>>,
    #[serde(
        alias = "additionalProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<ParametersOrU32<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum ParametersOrU32<'a> {
    U32(u32),
    #[serde(borrow)]
    Parameters(Parameters<'a>),
}

impl ParametersOrU32<'_> {
    pub fn allow_additional(&self) -> bool {
        if let ParametersOrU32::U32(v) = self {
            *v != 0
        } else {
            true
        }
    }
}
