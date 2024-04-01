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

#[derive(Debug, Deserialize, Serialize)]
pub struct Integer(
    #[serde(serialize_with = "serialize_int", deserialize_with = "deserialize_int")] pub u64,
);

impl Integer {
    pub fn get(&self) -> u64 {
        self.0
    }
}

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

pub fn deserialize_bool<'de, D>(d: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visitor;

    impl<'v> serde::de::Visitor<'v> for Visitor {
        type Value = bool;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "A string or number encoded bool")
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
            if let Ok(v) = u64::try_from(v) {
                Ok(v as f64)
            } else {
                Err(E::custom("Value out of range."))
            }
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
