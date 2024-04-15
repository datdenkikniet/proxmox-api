pub mod acme;
pub mod custom;
pub mod info;
pub struct CertificatesClient<T> {
    client: T,
    path: String,
}
impl<T> CertificatesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/certificates"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a CertificatesClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> CertificatesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Node index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
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
impl<T> CertificatesClient<T>
where
    T: crate::client::Client,
{
    pub fn acme(&self) -> acme::AcmeClient<T> {
        acme::AcmeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CertificatesClient<T>
where
    T: crate::client::Client,
{
    pub fn info(&self) -> info::InfoClient<T> {
        info::InfoClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> CertificatesClient<T>
where
    T: crate::client::Client,
{
    pub fn custom(&self) -> custom::CustomClient<T> {
        custom::CustomClient::<T>::new(self.client.clone(), &self.path)
    }
}
