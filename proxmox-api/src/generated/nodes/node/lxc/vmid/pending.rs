#[derive(Debug, Clone)]
pub struct PendingClient<T> {
    client: T,
    path: String,
}
impl<T> PendingClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/pending"),
        }
    }
}
impl<T> PendingClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get container configuration, including pending changes."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(key: String) -> Self {
        Self {
            key,
            delete: Default::default(),
            pending: Default::default(),
            value: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Indicates a pending delete request if present and not 0."]
    #[doc = ""]
    pub delete: Option<DeleteInt>,
    #[doc = "Configuration option name."]
    #[doc = ""]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pending value."]
    #[doc = ""]
    pub pending: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Current value."]
    #[doc = ""]
    pub value: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DeleteInt(i128);
impl crate::types::bounded_integer::BoundedInteger for DeleteInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = Some(2i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 0 and 2";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for DeleteInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for DeleteInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DeleteInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
