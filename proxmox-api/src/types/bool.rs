use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bool(
    #[serde(
        serialize_with = "serialize_bool",
        deserialize_with = "deserialize_bool"
    )]
    pub bool,
);

impl Bool {
    pub fn get(&self) -> bool {
        self.0
    }
}

pub fn deserialize_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'v> serde::de::Visitor<'v> for Visitor {
        type Value = bool;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a string or number encoded bool")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v == 0 {
                Ok(false)
            } else if v == 1 {
                Ok(true)
            } else {
                Err(E::custom("Invalid value for boolean"))
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if let Ok(value) = v.parse::<u64>() {
                if value == 0 {
                    Ok(false)
                } else if value == 1 {
                    Ok(true)
                } else {
                    Err(E::custom("invalid value for boolean"))
                }
            } else {
                Err(E::custom("invalid value for boolean"))
            }
        }
    }

    d.deserialize_any(Visitor)
}

pub fn serialize_bool<S>(value: &bool, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value = if !value { 0u32 } else { 1u32 };
    s.serialize_u32(value)
}

pub fn deserialize_bool_optional<'de, D>(d: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<Bool>::deserialize(d)?.map(|v| v.get()))
}

pub fn serialize_bool_optional<S>(value: &Option<bool>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.map(Bool).serialize(s)
}
