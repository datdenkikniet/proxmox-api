use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub struct Number(
    #[serde(
        serialize_with = "serialize_number",
        deserialize_with = "deserialize_number"
    )]
    pub f64,
);

impl Number {
    pub fn get(&self) -> f64 {
        self.0
    }
}
pub fn deserialize_number<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'v> serde::de::Visitor<'v> for Visitor {
        type Value = f64;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "A string or number encoded bool")
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as f64)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Ok(value) = v.parse::<f64>() {
                Ok(value)
            } else {
                Err(E::custom("invalid value for boolean"))
            }
        }
    }

    d.deserialize_any(Visitor)
}

pub fn serialize_number<S>(value: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f64(*value)
}

pub fn deserialize_number_optional<'de, D>(d: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<Number>::deserialize(d)?.map(|v| v.get()))
}

pub fn serialize_number_optional<S>(value: &Option<f64>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.map(Number).serialize(s)
}
