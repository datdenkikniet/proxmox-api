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

use serde::{Deserializer, Serializer};

pub fn serialize_bounded_integer<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: BoundedInteger,
{
    serializer.serialize_i128(value.get())
}

pub fn deserialize_bounded_integer<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: BoundedInteger + TryFrom<i128, Error = BoundedIntegerError>,
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
        T: BoundedInteger + TryFrom<i128, Error = BoundedIntegerError>,
    {
        type Value = T;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", T::TYPE_DESCRIPTION)
        }

        fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            T::try_from(v).map_err(|e| {
                E::custom(format!(
                    "could not parse {} as {} with error type {}",
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
            self.visit_i128(v as i128)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            self.visit_i128(v as i128)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let v_clone = &v;
            let parsed: i128 = v.parse().map_err(|e| {
                E::custom(format!(
                    "could not parse {} as {} with error type {}",
                    v_clone,
                    T::TYPE_DESCRIPTION,
                    e
                ))
            })?;
            self.visit_i128(parsed)
        }
    }

    deserializer.deserialize_any(Visitor::default())
}
