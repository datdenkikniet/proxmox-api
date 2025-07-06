pub mod controllers;
pub mod dns;
pub mod ipams;
pub mod vnets;
pub mod zones;
#[derive(Debug, Clone)]
pub struct SdnClient<T> {
    client: T,
    path: String,
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/sdn"),
        }
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Apply sdn controller changes && reload."]
    #[doc = ""]
    pub fn put(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(id: String) -> Self {
        Self {
            id,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub id: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn vnets(&self) -> vnets::VnetsClient<T> {
        vnets::VnetsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn zones(&self) -> zones::ZonesClient<T> {
        zones::ZonesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn controllers(&self) -> controllers::ControllersClient<T> {
        controllers::ControllersClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn ipams(&self) -> ipams::IpamsClient<T> {
        ipams::IpamsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> SdnClient<T>
where
    T: crate::client::Client,
{
    pub fn dns(&self) -> dns::DnsClient<T> {
        dns::DnsClient::<T>::new(self.client.clone(), &self.path)
    }
}
