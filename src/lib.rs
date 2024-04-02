mod path;
use std::collections::HashMap;

pub use path::{Path, PathElement};

mod api2;

mod client;
pub use client::{Client, Error};

mod serde;
pub use serde::*;

mod vmid;
pub use vmid::VmId;

pub struct AdditionalProperties(HashMap<String, serde_json::Value>);

impl ::serde::Serialize for AdditionalProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        todo!()
    }
}
