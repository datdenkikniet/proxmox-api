use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VmId(
    #[serde(
        deserialize_with = "crate::types::deserialize_int",
        serialize_with = "crate::types::serialize_int"
    )]
    u64,
);

impl VmId {
    pub fn new(value: u64) -> Option<Self> {
        if (100..=999_999_999).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for VmId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
