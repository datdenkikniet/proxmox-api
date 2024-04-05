use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub struct Integer(
    #[serde(serialize_with = "serialize_int", deserialize_with = "deserialize_int")] pub u64,
);

impl Integer {
    pub fn get(&self) -> u64 {
        self.0
    }
}

pub fn deserialize_int<'de, D>(d: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'v> serde::de::Visitor<'v> for Visitor {
        type Value = u64;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "A string or number encoded bool")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Ok(v) = u64::try_from(v) {
                Ok(v)
            } else {
                Err(E::custom("Value out of range."))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Ok(value) = v.parse::<u64>() {
                Ok(value)
            } else {
                Err(E::custom("invalid value for boolean"))
            }
        }
    }

    d.deserialize_any(Visitor)
}

pub fn serialize_int<S>(value: &u64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(*value)
}

pub fn deserialize_int_optional<'de, D>(d: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<Integer>::deserialize(d);
    Ok(v?.map(|v| v.get()))
}

pub fn serialize_int_optional<S>(value: &Option<u64>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.map(Integer).serialize(s)
}
