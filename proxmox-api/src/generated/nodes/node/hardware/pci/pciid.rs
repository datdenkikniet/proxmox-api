pub mod mdev;
#[derive(Debug, Clone)]
pub struct PciidClient<T> {
    client: T,
    path: String,
}
impl<T> PciidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, pciid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, pciid),
        }
    }
}
impl<T> PciidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index of available pci methods"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
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
impl<T> PciidClient<T>
where
    T: crate::client::Client,
{
    pub fn mdev(&self) -> mdev::MdevClient<T> {
        mdev::MdevClient::<T>::new(self.client.clone(), &self.path)
    }
}
