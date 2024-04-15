pub struct ZfsClient<T> {
    client: T,
    path: String,
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/zfs"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ZfsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Scan zfs pool list on local node."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(pool: String) -> Self {
        Self {
            pool,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "ZFS pool name."]
    #[doc = ""]
    pub pool: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
