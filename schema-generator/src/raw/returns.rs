use std::{borrow::Cow, collections::BTreeMap};

use serde::Deserialize;

use super::{Format, Items, Optional, Property};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Returns<'a> {
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    // If this is not set, validate that the object is empty in its entirety!
    #[serde(borrow, rename = "type")]
    pub ty: Option<Cow<'a, str>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub items: Option<Items<'a>>,
    #[serde(borrow, default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<Link<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<Cow<'a, str>, Property<'a>>>,
    #[serde(rename = "additionalProperties", default)]
    pub additional_properties: Option<u32>,
    #[serde(default, skip_serializing_if = "Optional::is_empty")]
    pub optional: Optional,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Format<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Link<'a> {
    pub href: Cow<'a, str>,
    pub rel: Cow<'a, str>,
}
