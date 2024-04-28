pub mod pci;
pub mod usb;
#[derive(Debug, Clone)]
pub struct MappingClient<T> {
    client: T,
    path: String,
}
impl<T> MappingClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/mapping"),
        }
    }
}
impl<T> MappingClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List resource types."]
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
impl<T> MappingClient<T>
where
    T: crate::client::Client,
{
    pub fn pci(&self) -> pci::PciClient<T> {
        pci::PciClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> MappingClient<T>
where
    T: crate::client::Client,
{
    pub fn usb(&self) -> usb::UsbClient<T> {
        usb::UsbClient::<T>::new(self.client.clone(), &self.path)
    }
}
