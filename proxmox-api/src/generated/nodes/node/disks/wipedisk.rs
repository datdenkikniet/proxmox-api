pub struct WipediskClient<T> {
    client: T,
    path: String,
}
impl<T> WipediskClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/wipedisk"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a WipediskClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> WipediskClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Wipe a disk or partition."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl PutParams {
    pub fn new(disk: String) -> Self {
        Self {
            disk,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[doc = "Block device name"]
    #[doc = ""]
    pub disk: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
