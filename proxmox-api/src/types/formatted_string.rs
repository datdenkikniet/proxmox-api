#[cfg(test)]
mod test {

    use serde::{de::Error, Deserialize};

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
            let is_multi_kvs = value.split_once(',').is_some() || value.split_once('=').is_some();
            let mut name = None;

            if !is_multi_kvs {
                return Ok(Test {
                    name: value.trim().to_string(),
                });
            } else {
                let values = value.split(',').map(|v| v.trim());

                for value in values {
                    let (k, v) = value
                        .split_once('=')
                        .ok_or(D::Error::custom("Missing key or value"))?;

                    let (k, v) = (k.trim(), v.trim());

                    match k {
                        "name" => {
                            if name.is_some() {
                                return Err(D::Error::custom(format!("Duplicate key {k}")));
                            }
                            let deserialized = ::std::str::FromStr::from_str(v)
                                .map_err(|e| D::Error::custom(format!("{e}")))?;
                            name = Some(deserialized);
                        }
                        k => return Err(D::Error::custom(format!("Unknown key {k}"))),
                    }
                }
            }

            let name = name.ok_or(D::Error::custom("Missing key 'name'"))?;

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
