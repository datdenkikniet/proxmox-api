pub struct VersionClient<T> {
    client: T,
    path: String,
}
impl<T> VersionClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/version"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a VersionClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> VersionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "API version details"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl GetOutput {
    pub fn new(release: String, repoid: String, version: String) -> Self {
        Self {
            release,
            repoid,
            version,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[doc = "The current installed Proxmox VE Release"]
    #[doc = ""]
    pub release: String,
    #[doc = "The short git commit hash ID from which this version was build"]
    #[doc = ""]
    pub repoid: String,
    #[doc = "The current installed pve-manager package version"]
    #[doc = ""]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
