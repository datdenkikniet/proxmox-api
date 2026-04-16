pub mod bridges;
pub mod content;
pub mod ip_vrf;
#[derive(Debug, Clone)]
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
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index for SDN zone status."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(subdir: String) -> Self {
        Self {
            subdir,
            additional_properties: ::std::default::Default::default(),
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
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    pub fn bridges(&self) -> bridges::BridgesClient<T> {
        bridges::BridgesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    pub fn ip_vrf(&self) -> ip_vrf::IpVrfClient<T> {
        ip_vrf::IpVrfClient::<T>::new(self.client.clone(), &self.path)
    }
}
