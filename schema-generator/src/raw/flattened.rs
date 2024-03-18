use std::{
    borrow::Cow,
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::Path;

use super::{Info, TreeNode};

#[derive(Clone)]
pub struct Value<'a> {
    pub leaf: Option<u32>,
    pub path: Path,
    pub text: Cow<'a, str>,
    pub info: BTreeMap<Cow<'a, str>, Info<'a>>,
}

impl core::fmt::Debug for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Value")
            .field("leaf", &self.leaf)
            .field("path", &format!("{}", &self.path))
            .field("text", &self.text)
            .field("info", &self.info)
            .finish()
    }
}

impl<'a> TryFrom<super::Value<'a>> for Value<'a> {
    type Error = ();

    fn try_from(value: super::Value<'a>) -> Result<Self, Self::Error> {
        let path = Path::try_from(value.path.as_ref())?;

        Ok(Self {
            leaf: value.leaf,
            path,
            text: value.text,
            info: value.info,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Collection<'a> {
    values: Vec<Value<'a>>,
}

impl<'a> Collection<'a> {
    fn push_nodes(list: &mut Vec<Value<'a>>, node: &TreeNode<'a>) -> Result<(), ()> {
        list.push(node.value.clone().try_into()?);

        for node in &node.children {
            Self::push_nodes(list, node)?;
        }

        Ok(())
    }

    pub fn from_nodes<'b>(nodes: &'b [TreeNode<'a>]) -> Result<Self, ()> {
        let mut values = Vec::with_capacity(nodes.len());

        for node in nodes {
            Self::push_nodes(&mut values, &node)?;
        }

        Ok(Self { values })
    }
}

impl<'a> Deref for Collection<'a> {
    type Target = [Value<'a>];

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> DerefMut for Collection<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
