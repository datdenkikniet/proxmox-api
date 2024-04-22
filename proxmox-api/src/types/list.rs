use serde::ser::Error;
use std::fmt::Write;
use std::marker::PhantomData;

use serde::{Deserializer, Serializer};

pub fn deserialize_list<'de, T, D, E>(d: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: TryFrom<String, Error = E>,
    E: std::fmt::Debug,
{
    struct Visitor<T, E> {
        _phantom: PhantomData<(E, T)>,
    }

    impl<T, P> Default for Visitor<T, P> {
        fn default() -> Self {
            Self {
                _phantom: Default::default(),
            }
        }
    }

    impl<'de, T, P> serde::de::Visitor<'de> for Visitor<T, P>
    where
        T: TryFrom<String, Error = P>,
        P: core::fmt::Debug,
    {
        type Value = Vec<T>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "a comma or semicolon separated list")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let data = v.split(',').map(str::trim);
            let mut values = Vec::new();

            for value in data {
                values.push(
                    TryFrom::try_from(value.to_string())
                        .map_err(|v| E::custom(format!("{v:?}")))?,
                );
            }

            Ok(values)
        }
    }

    d.deserialize_str(Visitor::default())
}

pub fn serialize_list<'a, T, S>(value: &[T], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: core::fmt::Display,
{
    let mut output = String::new();

    for v in value.iter() {
        if !output.is_empty() {
            output.push(',');
        }

        write!(output, "{v}").map_err(|e| S::Error::custom(e))?;
    }

    s.serialize_str(&output)
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    use crate::types::VmId;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Test {
        #[serde(
            serialize_with = "super::serialize_list",
            deserialize_with = "super::deserialize_list"
        )]
        pub ids: Vec<VmId>,
    }

    #[test]
    fn vmid_list() {
        let list = r#" { "ids": "101, 102, 103, 104" } "#;

        let test: Test = serde_json::from_str(list).unwrap();

        fn id(value: u64) -> VmId {
            VmId::new(value).unwrap()
        }

        assert_eq!(
            Test {
                ids: vec![id(101), id(102), id(103), id(104)]
            },
            test
        );
    }
}
