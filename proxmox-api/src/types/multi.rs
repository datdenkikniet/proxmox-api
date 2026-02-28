//! This module contains structures required to handle multi-fields, i.e.
//! `link[n]`. In these fields, the name of the field is dependent on the number,
//! so it requires some extra handling to get right.
//!
//! Additional data also needs extra handling in this case, which this module
//! also provides.

use std::{
    collections::{BTreeMap, HashMap},
    marker::PhantomData,
};

use serde::{Deserializer, Serialize, Serializer, de::DeserializeOwned, ser::SerializeMap};

pub trait NumberedItems: Default {
    type Item: DeserializeOwned + Serialize;
    const PREFIX: &'static str;

    fn key_matches(name: &str) -> bool {
        Self::check_key(name).is_some()
    }

    fn check_key(name: &str) -> Option<u32> {
        let (first_part, second_part) = name.split_once(Self::PREFIX)?;

        if first_part.is_empty() {
            let num = second_part.parse::<u32>().ok()?;
            Some(num)
        } else {
            None
        }
    }

    fn make_key(number: &u32) -> String {
        format!("{}{}", Self::PREFIX, number)
    }
}

pub trait Test {
    fn test_fn() -> fn(&str) -> bool;
}

pub fn deserialize_additional_data<'de, T, O, D>(d: D) -> Result<HashMap<String, O>, D::Error>
where
    D: Deserializer<'de>,
    T: Test,
    O: serde::de::Deserialize<'de>,
{
    struct Visitor<O, T> {
        _phantom: PhantomData<(O, T)>,
    }

    impl<O, T> Default for Visitor<O, T> {
        fn default() -> Self {
            Self {
                _phantom: Default::default(),
            }
        }
    }

    impl<'de, O, T> serde::de::Visitor<'de> for Visitor<O, T>
    where
        O: serde::de::Deserialize<'de>,
        T: Test,
    {
        type Value = HashMap<String, O>;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "additional properties")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>,
        {
            let mut output = HashMap::new();

            loop {
                let (key, value) = match map.next_entry::<&str, O>() {
                    Ok(Some(key)) => key,
                    Ok(None) => break,
                    Err(_) => continue,
                };

                if (T::test_fn())(key) {
                    continue;
                }

                output.insert(key.to_string(), value);
            }

            Ok(output)
        }
    }

    d.deserialize_map(Visitor::<O, T>::default())
}

macro_rules! define_multi_fns {
    ($([$type:ident, $ser:ident, $des:ident]),*) => {
        $(
            pub fn $ser<V, S>(value: &$type<u32, V::Item>, s: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                V: NumberedItems,
            {
                let mut map = s.serialize_map(Some(value.len()))?;

                for (key, value) in value.iter() {
                    map.serialize_entry(&V::make_key(key), value)?;
                }

                map.end()
            }

            pub fn $des<'de, V, D>(d: D) -> Result<$type<u32, V::Item>, D::Error>
            where
                V: NumberedItems,
                D: Deserializer<'de>,
            {
                struct InternalVisitor<V, D> {
                    _phantom: PhantomData<(D, V)>,
                }

                impl<V, D> Default for InternalVisitor<V, D> {
                    fn default() -> Self {
                        Self {
                            _phantom: Default::default(),
                        }
                    }
                }

                impl<'de, V, D> serde::de::Visitor<'de> for InternalVisitor<V, D>
                where
                    V: NumberedItems,
                    D: Deserializer<'de>,
                {
                    type Value = $type<u32, V::Item>;

                    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        write!(f, "Multi-numbered items with prefix {}", V::PREFIX)
                    }

                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        let mut output = $type::new();

                        while let Some(key) = map.next_key::<&str>()? {
                            let key = if let Some(idx) = V::check_key(key) {
                                idx
                            } else {
                                continue;
                            };

                            let value = map.next_value::<V::Item>()?;
                            output.insert(key, value);
                        }

                        Ok(output)
                    }
                }

                d.deserialize_map(InternalVisitor::<V, D>::default())
            }
        )*
    };
}

define_multi_fns!(
    [HashMap, serialize_multi, deserialize_multi],
    [BTreeMap, serialize_multi_btree, deserialize_multi_btree]
);

#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, HashMap};

    use serde::{Deserialize, Serialize};

    use super::{NumberedItems, Test};

    #[derive(Default)]
    struct NumberedNames;

    impl NumberedItems for NumberedNames {
        type Item = String;
        const PREFIX: &'static str = "name";
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Names {
        #[serde(
            flatten,
            deserialize_with = "super::deserialize_multi_btree::<'_, NumberedNames, _>",
            serialize_with = "super::serialize_multi_btree::<NumberedNames, _>"
        )]
        names: BTreeMap<u32, String>,
        #[serde(
            flatten,
            deserialize_with = "super::deserialize_additional_data::<'_, Names, _, _>"
        )]
        additional_data: HashMap<String, String>,
    }

    impl Test for Names {
        fn test_fn() -> fn(&str) -> bool {
            fn the_test(input: &str) -> bool {
                ["names"].contains(&input) || NumberedNames::key_matches(input)
            }

            the_test
        }
    }

    fn map_kv<K, V, NK, NV>((k, v): (K, V)) -> (NK, NV)
    where
        K: Into<NK>,
        V: Into<NV>,
    {
        (k.into(), v.into())
    }

    #[test]
    pub fn deserialize_multi() {
        let text = r#"{ "name1": "value1", "name2": "value2", "name_4": 0, "name_3": false, "name_2": "additional" }"#;

        let names: Names = serde_json::from_str(text).unwrap();
        let expected = Names {
            names: [(1u32, "value1"), (2, "value2")]
                .into_iter()
                .map(map_kv)
                .collect(),
            additional_data: [("name_2", "additional")].into_iter().map(map_kv).collect(),
        };

        assert_eq!(names, expected);
    }

    #[test]
    fn serialize_mluti() {
        let names = Names {
            names: [(1u32, "value1"), (2, "value2")]
                .into_iter()
                .map(map_kv)
                .collect(),
            additional_data: [("name_2", "additional")].into_iter().map(map_kv).collect(),
        };

        let expected = r#"{"name1":"value1","name2":"value2","name_2":"additional"}"#;
        let json = serde_json::to_string(&names).unwrap();
        assert_eq!(json, expected);
    }
}
