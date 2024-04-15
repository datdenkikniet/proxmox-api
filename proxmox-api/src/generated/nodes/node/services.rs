pub mod service;
pub struct ServicesClient<T> {
    client: T,
    path: String,
}
impl<T> ServicesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/services"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ServicesClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ServicesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Service list."]
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
impl<T> ServicesClient<T>
where
    T: crate::client::Client,
{
    pub fn service(&self, service: &str) -> service::ServiceClient<T> {
        service::ServiceClient::<T>::new(self.client.clone(), &self.path, service)
    }
}
