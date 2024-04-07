use std::str::FromStr;

use serde::{de::Error, Deserializer};

pub enum Output<O, I> {
    Single(O),
    Iter(I),
}

pub fn get_keys_or_default_value<'de, D, O, E>(
    value: &str,
) -> Result<Output<O, impl Iterator<Item = Result<(&str, &str), D::Error>>>, D::Error>
where
    O: FromStr<Err = E>,
    D: Deserializer<'de>,
    E: core::fmt::Display,
{
    // TODO: escape strings and stuff
    let is_multi_kvs = value.split_once(',').is_some() || value.split_once('=').is_some();

    let output = if !is_multi_kvs {
        Output::Single(FromStr::from_str(value.trim()).map_err(D::Error::custom)?)
    } else {
        let iter = value.split(',').map(|v| {
            let (k, v) = v
                .split_once('=')
                .ok_or_else(|| "missing key or value")
                .map_err(D::Error::custom)?;
            Ok((k.trim(), v.trim()))
        });

        Output::Iter(iter)
    };

    Ok(output)
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use serde::{de::Error, Deserialize};

    use super::get_keys_or_default_value;

    #[derive(Debug, PartialEq)]
    pub struct Test {
        name: String,
    }

    impl<'de> Deserialize<'de> for Test {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = <&str>::deserialize(deserializer)?;
            let values = get_keys_or_default_value::<D, _, _>(value)?;

            let iter = match values {
                super::Output::Single(name) => return Ok(Test { name }),
                super::Output::Iter(iter) => iter,
            };

            let mut name = None;

            for value in iter {
                let (k, v) = value.map_err(D::Error::custom)?;
                match k {
                    "name" => {
                        if name.is_some() {
                            return Err(D::Error::custom("Duplicate key 'name'"));
                        }
                        name = Some(FromStr::from_str(v).map_err(D::Error::custom)?)
                    }
                    key => return Err(D::Error::custom(format!("Unknown key {key}"))),
                }
            }

            let name = name
                .ok_or_else(|| "Missing required key 'name'")
                .map_err(D::Error::custom)?;

            Ok(Test { name })
        }
    }

    #[test]
    pub fn from_default() {
        let data = r#""name""#;
        let unwrapped: Test = serde_json::from_str(data).unwrap();
        assert_eq!(
            unwrapped,
            Test {
                name: "name".into()
            }
        );
    }

    #[test]
    pub fn from_kv() {
        let data = r#""name=name""#;
        let unwrapped: Test = serde_json::from_str(data).unwrap();
        assert_eq!(
            unwrapped,
            Test {
                name: "name".into()
            }
        );
    }
}
