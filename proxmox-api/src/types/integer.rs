use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub struct Integer(
    #[serde(serialize_with = "serialize_int", deserialize_with = "deserialize_int")] pub i64,
);

impl Integer {
    pub fn get(&self) -> i64 {
        self.0
    }
}

pub fn deserialize_int<'de, D>(d: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'v> serde::de::Visitor<'v> for Visitor {
        type Value = i64;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "A string or number encoded bool")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as i64)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Ok(value) = v.parse::<i64>() {
                Ok(value)
            } else {
                Err(E::custom("invalid value for boolean"))
            }
        }
    }

    d.deserialize_any(Visitor)
}

pub fn serialize_int<S>(value: &i64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i64(*value)
}

pub fn deserialize_int_optional<'de, D>(d: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<Integer>::deserialize(d);
    Ok(v?.map(|v| v.get()))
}

pub fn serialize_int_optional<S>(value: &Option<i64>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.map(Integer).serialize(s)
}
