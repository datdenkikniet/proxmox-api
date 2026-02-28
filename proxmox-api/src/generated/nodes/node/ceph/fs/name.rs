#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a Ceph filesystem"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "add-storage")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure the created CephFS as storage for this cluster."]
    #[doc = ""]
    pub add_storage: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of placement groups for the backing data pool. The metadata pool will use a quarter of this."]
    #[doc = ""]
    pub pg_num: Option<PgNumInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PgNumInt(i128);
impl crate::types::bounded_integer::BoundedInteger for PgNumInt {
    const MIN: Option<i128> = Some(8i128);
    const MAX: Option<i128> = Some(32768i128);
    const DEFAULT: Option<i128> = Some(128i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 8 and 32768";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for PgNumInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for PgNumInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PgNumInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
