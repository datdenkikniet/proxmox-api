#[derive(Debug, Clone)]
pub struct MigrateClient<T> {
    client: T,
    path: String,
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/migrate"),
        }
    }
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migrate the container to another node. Creates a new migration task."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(target: String) -> Self {
        Self {
            target,
            bwlimit: Default::default(),
            online: Default::default(),
            restart: Default::default(),
            target_storage: Default::default(),
            timeout: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<BwlimitNum>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use online/live migration."]
    #[doc = ""]
    pub online: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use restart migration"]
    #[doc = ""]
    pub restart: Option<bool>,
    #[doc = "Target node."]
    #[doc = ""]
    pub target: String,
    #[serde(rename = "target-storage")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[doc = ""]
    pub target_storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Timeout in seconds for shutdown for restart migration"]
    #[doc = ""]
    pub timeout: Option<TimeoutInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TimeoutInt(i128);
impl crate::types::bounded_integer::BoundedInteger for TimeoutInt {
    const MIN: Option<i128> = None::<i128>;
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(180i128);
    const TYPE_DESCRIPTION: &'static str = "a valid integer";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for TimeoutInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for TimeoutInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for TimeoutInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BwlimitNum(f64);
impl crate::types::bounded_number::BoundedNumber for BwlimitNum {
    const MIN: Option<f64> = Some(0f64);
    const MAX: Option<f64> = None::<f64>;
    const DEFAULT: Option<f64> = None::<f64>;
    const TYPE_DESCRIPTION: &'static str = "an number greater than or equal to 0";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for BwlimitNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for BwlimitNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for BwlimitNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for BwlimitNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for BwlimitNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for BwlimitNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_number(deserializer)
    }
}
