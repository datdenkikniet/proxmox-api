pub mod flags;
pub mod metadata;
pub mod status;
#[derive(Debug, Clone)]
pub struct CephClient<T> {
    client: T,
    path: String,
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ceph"),
        }
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Cluster ceph index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn metadata(&self) -> metadata::MetadataClient<T> {
        metadata::MetadataClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CephClient<T>
where
    T: crate::client::Client,
{
    pub fn flags(&self) -> flags::FlagsClient<T> {
        flags::FlagsClient::<T>::new(self.client.clone(), &self.path)
    }
}
