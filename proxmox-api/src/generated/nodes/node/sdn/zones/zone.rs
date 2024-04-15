pub mod content;
pub struct ZoneClient<T> {
    client: T,
    path: String,
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, zone: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, zone),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ZoneClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(subdir: String) -> Self {
        Self {
            subdir,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub subdir: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    pub fn content(&self) -> content::ContentClient<T> {
        content::ContentClient::<T>::new(self.client.clone(), &self.path)
    }
}
