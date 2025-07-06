#[derive(Debug, Clone)]
pub struct StartallClient<T> {
    client: T,
    path: String,
}
impl<T> StartallClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/startall"),
        }
    }
}
impl<T> StartallClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Start all VMs and containers located on this node (by default only those with onboot=1)."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Issue start command even if virtual guest have 'onboot' not set or set to off."]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only consider guests from this comma separated list of VMIDs."]
    #[doc = ""]
    pub vms: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
