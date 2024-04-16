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
impl<'a, T> crate::ProxmoxClient for &'a MtunnelClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> MtunnelClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migration tunnel endpoint - only for internal use by CT migration."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, (), T::Error> for &MtunnelClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<(), T::Error> {
        self.post(params)
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
