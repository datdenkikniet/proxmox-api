pub enum Output<O, I> {
    Single(O),
    Iter(I),
}

pub fn get_keys_or_default_value<'str, O>(
    value: &'str str,
) -> Result<Output<O, impl Iterator<Item = Result<(&str, &str), String>>>, O::Error>
where
    O: TryFrom<&'str str>,
{
    // TODO: escape strings and stuff
    let is_multi_kvs = value.split_once(',').is_some() || value.split_once('=').is_some();

    let output = if !is_multi_kvs {
        Output::Single(TryFrom::try_from(value.trim())?)
    } else {
        let iter = value.split(',').map(|v| {
            let (k, v) = v.split_once('=').ok_or_else(|| "missing key or value")?;
            Ok((k.trim(), v.trim()))
        });

        Output::Iter(iter)
    };

    Ok(output)
}

#[cfg(test)]
mod test {

    use serde::{Deserialize, Serialize};

    use super::get_keys_or_default_value;

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(try_from = "&str", into = "String")]
    pub struct Test {
        name: String,
    }

    impl TryFrom<&str> for Test {
        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let values = get_keys_or_default_value(value).unwrap();

            let iter = match values {
                super::Output::Single(name) => return Ok(Test { name }),
                super::Output::Iter(iter) => iter,
            };

            let mut name = None;

            for value in iter {
                let (k, v) = value?;
                match k {
                    "name" => {
                        if name.is_some() {
                            return Err("Duplicate key 'name'".into());
                        }
                        name = Some(TryFrom::try_from(v).unwrap())
                    }
                    key => return Err(format!("Unknown key {key}")),
                }
            }

            let name = name.ok_or_else(|| "Missing required key 'name'".to_string())?;

            Ok(Test { name })
        }
    }

    impl From<Test> for String {
        fn from(value: Test) -> Self {
            todo!()
        }
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
