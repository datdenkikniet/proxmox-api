use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use super::ty::Type;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Returns<'a> {
    #[serde(borrow, flatten, default, skip_serializing_if = "Option::is_none")]
    pub ty: Option<Type<'a>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<Link<'a>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Link<'a> {
    pub href: Cow<'a, str>,
    pub rel: Cow<'a, str>,
}
