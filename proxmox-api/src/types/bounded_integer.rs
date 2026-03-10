#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedIntegerError {
    ValueLower,
    ValueHigher,
}

impl std::fmt::Display for BoundedIntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundedIntegerError::ValueLower => write!(f, "value is below minimum"),
            BoundedIntegerError::ValueHigher => write!(f, "value is above maximum"),
        }
    }
}

impl std::error::Error for BoundedIntegerError {}

pub trait BoundedInteger {
    const MIN: Option<i128> = None;
    const MAX: Option<i128> = None;
    const DEFAULT: Option<i128> = None;
    const TYPE_DESCRIPTION: &'static str;

    fn get(&self) -> i128;

    fn new(value: i128) -> Result<Self, BoundedIntegerError>
    where
        Self: Sized;

    fn validate(value: i128) -> Result<(), BoundedIntegerError> {
        if let Some(min) = Self::MIN
            && value < min
        {
            return Err(BoundedIntegerError::ValueLower);
        }
        if let Some(max) = Self::MAX
            && value > max
        {
            return Err(BoundedIntegerError::ValueHigher);
        }
        Ok(())
    }
}

use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_bounded_integer<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: BoundedInteger,
{
    serializer.serialize_i128(value.get())
}

fn parse_value_to_i128(value: &serde_json::Value) -> Option<i128> {
    match value {
        serde_json::Value::Number(n) => n.as_i128(),
        serde_json::Value::String(s) => s.parse::<i128>().ok(),
        _ => None,
    }
}

pub fn deserialize_bounded_integer<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: BoundedInteger + TryFrom<i128, Error = BoundedIntegerError>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    let i128_value = value
        .as_ref()
        .and_then(parse_value_to_i128)
        .ok_or_else(|| serde::de::Error::custom("could not parse value as i128"))?;
    T::try_from(i128_value).map_err(|e| {
        serde::de::Error::custom(format!(
            "could not parse as {} with error: {}",
            T::TYPE_DESCRIPTION,
            e
        ))
    })
}
