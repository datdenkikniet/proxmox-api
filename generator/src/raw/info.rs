use std::borrow::Cow;

use serde::Deserialize;

use super::{Parameters, Permission, Type};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Info<'a> {
    #[serde(
        rename = "allowtoken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_token: Option<u32>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub method: Cow<'a, str>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,
    #[serde(borrow, default, skip_serializing_if = "Option::is_none")]
    pub returns: Option<Type<'a>>,
    pub protected: Option<u32>,
    #[serde(
        rename = "proxyto",
        borrow,
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proxy_to: Option<Cow<'a, str>>,
    #[serde(default)]
    pub download_allowed: Option<u32>,
}
