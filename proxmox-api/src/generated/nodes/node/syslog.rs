pub struct SyslogClient<T> {
    client: T,
    path: String,
}
impl<T> SyslogClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/syslog"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Service ID"]
    pub service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display all log since this date-time string."]
    pub since: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display all log until this date-time string."]
    pub until: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(n: u64, t: String) -> Self {
        Self {
            n,
            t,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "Line number"]
    pub n: u64,
    #[doc = "Line text"]
    pub t: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SyslogClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read system log"]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
