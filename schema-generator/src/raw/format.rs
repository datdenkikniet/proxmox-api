use std::{
    borrow::Cow,
    collections::{BTreeMap, HashSet},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Format<'a> {
    #[serde(borrow)]
    StringDescription(Cow<'a, str>),
    #[serde(borrow)]
    Properties(BTreeMap<Cow<'a, str>, FormatProperty<'a>>),
}

impl Format<'_> {
    pub fn is_pve_prev_list(&self) -> bool {
        matches!(
            self,
            Self::StringDescription(Cow::Borrowed("pve-prev-list"))
        )
    }
}

impl PartialEq<str> for Format<'_> {
    fn eq(&self, other: &str) -> bool {
        if let Self::StringDescription(str) = self {
            str == other
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FormatProperty<'a> {
    // This is practically always `minimum`, but only in the next-id case is it `min`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<u32>,
    // This is practically always `maximum`, but only in the next-id case is it `max`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<u32>,
    // Practically always a u32, but sometimes in string form, because
    // god knows why
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u32>,
    // Practically always u32, but not always, of course...
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<u32>,
    #[serde(rename = "type", borrow)]
    pub ty: Option<Cow<'a, str>>,
    #[serde(rename = "typetext", default, skip_serializing_if = "Option::is_none")]
    pub ty_text: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Box<Format<'a>>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub format_description: Option<Cow<'a, str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_key: Option<u32>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<Cow<'a, str>>,
    #[serde(
        borrow,
        rename = "enum",
        default,
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub enum_values: HashSet<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub verbose_description: Option<Cow<'a, str>>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<Cow<'a, str>>,
    #[serde(
        alias = "keyAlias",
        borrow,
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub key_alias: Option<Cow<'a, str>>,
    #[serde(alias = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u32>,
    #[serde(alias = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u32>,
}
