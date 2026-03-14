use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::num::NonZeroU64;

#[derive(Debug, Deserialize, Serialize)]
pub struct NonZeroUnsignedInteger(
    #[serde(
        serialize_with = "serialize_non_zero_pos_int",
        deserialize_with = "deserialize_non_zero_pos_int"
    )]
    pub NonZeroU64,
);

impl NonZeroUnsignedInteger {
    pub fn get(&self) -> NonZeroU64 {
        self.0
    }
}

pub fn deserialize_non_zero_pos_int<'de, D>(d: D) -> Result<NonZeroU64, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'v> serde::de::Visitor<'v> for Visitor {
        type Value = NonZeroU64;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "a non-zero positive integer or string encoding a non-zero positive integer"
            )
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            NonZeroU64::new(v).ok_or_else(|| E::custom("value must be non-zero"))
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v <= 0 {
                Err(E::custom("value must be positive and non-zero"))
            } else {
                NonZeroU64::new(v as u64).ok_or_else(|| E::custom("value must be non-zero"))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Ok(value) = v.parse::<u64>() {
                NonZeroU64::new(value).ok_or_else(|| E::custom("value must be non-zero"))
            } else {
                Err(E::custom("invalid value for non-zero positive integer"))
            }
        }
    }

    d.deserialize_any(Visitor)
}

pub fn serialize_non_zero_pos_int<S>(value: &NonZeroU64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(value.get())
}

pub fn deserialize_non_zero_pos_int_optional<'de, D>(d: D) -> Result<Option<NonZeroU64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<NonZeroUnsignedInteger>::deserialize(d);
    Ok(v?.map(|v| v.get()))
}

pub fn serialize_non_zero_pos_int_optional<S>(
    value: &Option<NonZeroU64>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.map(NonZeroUnsignedInteger).serialize(s)
}
