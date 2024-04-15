pub struct GetFsinfoClient<T> {
    client: T,
    path: String,
}
impl<T> GetFsinfoClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/get-fsinfo"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a GetFsinfoClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> GetFsinfoClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Execute get-fsinfo."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
