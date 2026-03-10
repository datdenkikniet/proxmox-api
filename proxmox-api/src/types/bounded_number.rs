#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundedNumberError {
    ValueLower,
    ValueHigher,
    NotANumber,
}

impl std::fmt::Display for BoundedNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundedNumberError::ValueLower => write!(f, "value is below minimum"),
            BoundedNumberError::ValueHigher => write!(f, "value is above maximum"),
            BoundedNumberError::NotANumber => write!(f, "value is not a number (NaN)"),
        }
    }
}

impl std::error::Error for BoundedNumberError {}

pub trait BoundedNumber {
    const MIN: Option<f64> = None;
    const MAX: Option<f64> = None;
    const DEFAULT: Option<f64> = None;
    const TYPE_DESCRIPTION: &'static str;

    fn get(&self) -> f64;

    fn new(value: f64) -> Result<Self, BoundedNumberError>
    where
        Self: Sized;

    fn validate(value: f64) -> Result<(), BoundedNumberError> {
        if value.is_nan() {
            return Err(BoundedNumberError::NotANumber);
        }
        if let Some(min) = Self::MIN
            && value < min
        {
            return Err(BoundedNumberError::ValueLower);
        }
        if let Some(max) = Self::MAX
            && value > max
        {
            return Err(BoundedNumberError::ValueHigher);
        }
        Ok(())
    }
}

use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize_bounded_number<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: BoundedNumber,
{
    serializer.serialize_f64(value.get())
}

fn parse_value_to_f64(value: &serde_json::Value) -> Option<f64> {
    match value {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => s.parse::<f64>().ok(),
        _ => None,
    }
}

pub fn deserialize_bounded_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: BoundedNumber + TryFrom<f64, Error = BoundedNumberError>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    let f64_value = value
        .as_ref()
        .and_then(parse_value_to_f64)
        .ok_or_else(|| serde::de::Error::custom("could not parse value as f64"))?;
    T::try_from(f64_value).map_err(|e| {
        serde::de::Error::custom(format!(
            "could not parse as {} with error: {}",
            T::TYPE_DESCRIPTION,
            e
        ))
    })
}
