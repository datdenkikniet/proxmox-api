use serde::{Deserialize, Deserializer, Serialize};

pub mod access;
pub mod cluster;
pub mod nodes;
pub mod pools;

mod types;
pub use types::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bool(pub bool);

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        value.0
    }
}

impl Serialize for Bool {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = if self.0 { 1 } else { 0 };
        serializer.serialize_u32(value)
    }
}

impl<'de> Deserialize<'de> for Bool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Bool;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "Unsigned integer with value 0 or 1, indicating bool"
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Ok(value) = v.parse::<u32>() {
                    if value == 0 {
                        Ok(Bool(false))
                    } else if value == 1 {
                        Ok(Bool(true))
                    } else {
                        Err(E::custom("Invalid value for bool"))
                    }
                } else {
                    Err(E::custom("Could not parse input as u32"))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value == 0 {
                    Ok(Bool(false))
                } else if value == 1 {
                    Ok(Bool(true))
                } else {
                    Err(E::custom("Invalid value for bool"))
                }
            }
        }

        deserializer.deserialize_bool(Visitor)
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct VmId(u32);

impl std::fmt::Display for VmId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl VmId {
    pub fn new(value: u32) -> Option<Self> {
        if value >= 100 && value <= 999999999u32 {
            Some(VmId(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl<'de> Deserialize<'de> for VmId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl serde::de::Visitor<'_> for Visitor {
            type Value = VmId;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "integer or string containing integer")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if let Ok(value) = v.parse::<u32>() {
                    VmId::new(value).ok_or(E::custom("VM ID was out of range."))
                } else {
                    Err(E::custom("Could not parse input as i64"))
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.try_into()
                    .ok()
                    .map(VmId::new)
                    .flatten()
                    .ok_or(E::custom("VM ID was out of range."))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VmStatus {
    Stopped,
    Running,
}
