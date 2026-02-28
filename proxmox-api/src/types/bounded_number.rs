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

use serde::{Deserializer, Serializer};

pub fn serialize_bounded_number<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: BoundedNumber,
{
    serializer.serialize_f64(value.get())
}

pub fn deserialize_bounded_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: BoundedNumber + TryFrom<f64, Error = BoundedNumberError>,
{
    struct Visitor<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Default for Visitor<T> {
        fn default() -> Self {
            Self {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
    where
        T: BoundedNumber + TryFrom<f64, Error = BoundedNumberError>,
    {
        type Value = T;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", T::TYPE_DESCRIPTION)
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            T::try_from(v).map_err(|e| {
                E::custom(format!(
                    "invalid value {} for {} with error type: {}",
                    v,
                    T::TYPE_DESCRIPTION,
                    e
                ))
            })
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_f64(v as f64)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_f64(v as f64)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let v_clone = &v;
            let parsed: f64 = v.parse().map_err(|e| {
                E::custom(format!(
                    "could not parse {} as {} with error type {}",
                    v_clone,
                    T::TYPE_DESCRIPTION,
                    e
                ))
            })?;
            self.visit_f64(parsed)
        }
    }

    deserializer.deserialize_any(Visitor::default())
}
