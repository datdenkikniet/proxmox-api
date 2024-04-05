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
    #[doc = "Service list."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> ServicesClient<T>
where
    T: crate::client::Client,
{
    pub fn service(&self, service: &str) -> service::ServiceClient<T> {
        service::ServiceClient::<T>::new(self.client.clone(), &self.path, service)
    }
}
