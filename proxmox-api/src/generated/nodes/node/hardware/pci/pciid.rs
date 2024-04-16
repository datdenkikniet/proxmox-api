pub mod mdev;
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
impl<'a, T> crate::ProxmoxClient for &'a PciidClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> PciidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index of available pci methods"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &PciidClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
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
