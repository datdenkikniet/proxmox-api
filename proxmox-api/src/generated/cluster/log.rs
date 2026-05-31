#[derive(Debug, Clone)]
pub struct LogClient<T> {
    client: T,
    path: String,
}
impl<T> LogClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/log"),
        }
    }
}
impl<T> LogClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read cluster log"]
    #[doc = ""]
    #[doc = "The user needs 'Sys.Syslog' on '/' in order to get all logs."]
    pub async fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &params).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_non_zero_pos_int_optional",
        deserialize_with = "crate::types::deserialize_non_zero_pos_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum number of entries."]
    #[doc = ""]
    pub max: Option<std::num::NonZeroU64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
