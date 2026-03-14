use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub struct UnsignedInteger(
    #[serde(
        serialize_with = "serialize_unsigned_int",
        deserialize_with = "deserialize_unsigned_int"
    )]
    pub u64,
);

impl UnsignedInteger {
    pub fn get(&self) -> u64 {
        self.0
    }
}

pub fn deserialize_unsigned_int<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'v> serde::de::Visitor<'v> for Visitor {
        type Value = u64;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "an unsigned integer or string encoding an unsigned integer"
            )
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v < 0 {
                Err(E::custom("invalid value for unsigned integer"))
            } else {
                Ok(v as u64)
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Ok(value) = v.parse::<u64>() {
                Ok(value)
            } else {
                Err(E::custom("invalid value for unsigned integer"))
            }
        }
    }

    d.deserialize_any(Visitor)
}

pub fn serialize_unsigned_int<S>(value: &u64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(*value)
}

pub fn deserialize_unsigned_int_optional<'de, D>(d: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<UnsignedInteger>::deserialize(d);
    Ok(v?.map(|v| v.get()))
}

pub fn serialize_unsigned_int_optional<S>(value: &Option<u64>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.map(UnsignedInteger).serialize(s)
}
