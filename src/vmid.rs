use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VmId(
    #[serde(
        deserialize_with = "crate::deserialize_int",
        serialize_with = "crate::serialize_int"
    )]
    u64,
);

impl VmId {
    pub fn get(&self) -> u64 {
        self.0
    }
}
