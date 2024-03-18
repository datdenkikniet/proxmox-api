use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
};

use serde::Deserialize;

use super::{Format, Property};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Items<'a> {
    #[serde(rename = "type", borrow)]
    pub ty: Cow<'a, str>,
    #[serde(
        rename = "additionalProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<Cow<'a, str>, Property<'a>>,
    // Wat, nexted items?
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Items<'a>>>,
    #[serde(
        borrow,
        rename = "enum",
        default,
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub enum_values: HashSet<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Format<'a>>,
    #[serde(alias = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
    #[serde(alias = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
}
