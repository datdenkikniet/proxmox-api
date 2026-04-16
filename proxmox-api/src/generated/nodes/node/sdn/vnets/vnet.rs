pub mod mac_vrf;
#[derive(Debug, Clone)]
pub struct VnetClient<T> {
    client: T,
    path: String,
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, vnet: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, vnet),
        }
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
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
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    pub fn mac_vrf(&self) -> mac_vrf::MacVrfClient<T> {
        mac_vrf::MacVrfClient::<T>::new(self.client.clone(), &self.path)
    }
}
