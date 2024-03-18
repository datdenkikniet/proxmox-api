use serde::Deserialize;
use std::{borrow::Cow, collections::BTreeMap};

pub mod flattened;

mod format;
pub use format::{Format, FormatProperty};

mod info;
pub use info::Info;

mod items;
pub use items::Items;

mod parameters;
pub use parameters::Parameters;

mod permission;
pub use permission::Permission;

mod property;
pub use property::{ParametersOrU32, Property};

mod returns;
pub use returns::Returns;

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TreeNode<'a> {
    #[serde(borrow, flatten)]
    pub value: Value<'a>,
    #[serde(borrow, default)]
    pub children: Vec<TreeNode<'a>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Value<'a> {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaf: Option<u32>,
    #[serde(borrow)]
    pub path: Cow<'a, str>,
    #[serde(borrow)]
    pub text: Cow<'a, str>,
    #[serde(borrow, default)]
    pub info: BTreeMap<Cow<'a, str>, Info<'a>>,
}
