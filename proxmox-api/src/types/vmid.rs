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

impl TryFrom<String> for VmId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: u64 = value
            .parse()
            .map_err(|_| format!("Invalid VM id '{value}'"))?;

        Self::new(value).ok_or_else(|| format!("Invalid VM id '{value}' (out of range)"))
    }
}
