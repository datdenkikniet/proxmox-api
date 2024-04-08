pub struct MtunnelClient<T> {
    client: T,
    path: String,
}
impl<T> MtunnelClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/mtunnel"),
        }
    }
}
impl<T> MtunnelClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migration tunnel endpoint - only for internal use by VM migration."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of network bridges to check availability. Will be checked again for actually used bridges during migration."]
    #[doc = ""]
    pub bridges: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of storages to check permission and availability. Will be checked again for all actually used storages during migration."]
    #[doc = ""]
    pub storages: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
