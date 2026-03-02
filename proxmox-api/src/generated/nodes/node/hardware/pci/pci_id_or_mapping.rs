pub mod mdev;
#[derive(Debug, Clone)]
pub struct PciIdOrMappingClient<T> {
    client: T,
    path: String,
}
impl<T> PciIdOrMappingClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, pci_id_or_mapping: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, pci_id_or_mapping),
        }
    }
}
impl<T> PciIdOrMappingClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index of available pci methods"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(method: String) -> Self {
        Self {
            method,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub method: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(pci_id_or_mapping: String) -> Self {
        Self {
            pci_id_or_mapping,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(rename = "pci-id-or-mapping")]
    pub pci_id_or_mapping: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> PciIdOrMappingClient<T>
where
    T: crate::client::Client,
{
    pub fn mdev(&self) -> mdev::MdevClient<T> {
        mdev::MdevClient::<T>::new(self.client.clone(), &self.path)
    }
}
