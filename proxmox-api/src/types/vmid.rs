use super::bounded_integer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VmId(i128);

impl bounded_integer::BoundedInteger for VmId {
    const MIN: Option<i128> = Some(100);
    const MAX: Option<i128> = Some(999_999_999);
    const DEFAULT: Option<i128> = None;
    const TYPE_DESCRIPTION: &'static str = "an integer between 100 and 999_999_999";

    fn get(&self) -> i128 {
        self.0
    }

    fn new(value: i128) -> Result<Self, bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}

impl std::convert::TryFrom<i128> for VmId {
    type Error = bounded_integer::BoundedIntegerError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        bounded_integer::BoundedInteger::new(value)
    }
}

impl ::serde::Serialize for VmId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        bounded_integer::serialize_bounded_integer(self, serializer)
    }
}

impl<'de> ::serde::Deserialize<'de> for VmId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
impl std::fmt::Display for VmId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
