use std::{borrow::Cow, collections::BTreeMap};

use serde::Deserialize;

use super::Property;

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct Parameters<'a> {
    #[serde(rename = "additionalProperties", default)]
    pub additional_properties: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<Cow<'a, str>, Property<'a>>,
    #[serde(rename = "type", borrow, default)]
    pub ty: Cow<'a, str>,
}

impl Parameters<'_> {
    pub fn is_empty(&self) -> bool {
        self.ty.is_empty() && self.additional_properties.is_none() && self.properties.is_empty()
    }
}
