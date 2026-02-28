#[derive(Debug, Clone)]
pub struct StopallClient<T> {
    client: T,
    path: String,
}
impl<T> StopallClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/stopall"),
        }
    }
}
impl<T> StopallClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Stop all VMs and Containers."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "force-stop")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force a hard-stop after the timeout."]
    #[doc = ""]
    pub force_stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Timeout for each guest shutdown task. Depending on `force-stop`, the shutdown gets then simply aborted or a hard-stop is forced."]
    #[doc = ""]
    pub timeout: Option<TimeoutInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only consider Guests with these IDs."]
    #[doc = ""]
    pub vms: Option<String>,
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
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = Some(7200i128);
    const DEFAULT: Option<i128> = Some(180i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 0 and 7200";
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
