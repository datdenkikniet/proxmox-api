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
impl<'a, T> crate::ProxmoxClient for &'a NameClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Remove an LVM thin pool."]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.delete(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<DeleteParams, String, T::Error>
    for &NameClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Delete;
    fn exec(&self, params: DeleteParams) -> Result<String, T::Error> {
        self.delete(params)
    }
}
impl DeleteParams {
    pub fn new(volume_group: String) -> Self {
        Self {
            volume_group,
            cleanup_config: Default::default(),
            cleanup_disks: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct DeleteParams {
    #[serde(rename = "cleanup-config")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Marks associated storage(s) as not available on this node anymore or removes them from the configuration (if configured for this node only)."]
    #[doc = ""]
    pub cleanup_config: Option<bool>,
    #[serde(rename = "cleanup-disks")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Also wipe disks so they can be repurposed afterwards."]
    #[doc = ""]
    pub cleanup_disks: Option<bool>,
    #[serde(rename = "volume-group")]
    #[doc = "The storage identifier."]
    #[doc = ""]
    pub volume_group: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
