pub mod migrate;
pub mod shutdown;
pub mod start;
pub mod suspend;
#[derive(Debug, Clone)]
pub struct GuestClient<T> {
    client: T,
    path: String,
}
impl<T> GuestClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/guest"),
        }
    }
}
impl<T> GuestClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Bulk action index."]
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
impl<T> GuestClient<T>
where
    T: crate::client::Client,
{
    pub fn start(&self) -> start::StartClient<T> {
        start::StartClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> GuestClient<T>
where
    T: crate::client::Client,
{
    pub fn shutdown(&self) -> shutdown::ShutdownClient<T> {
        shutdown::ShutdownClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> GuestClient<T>
where
    T: crate::client::Client,
{
    pub fn suspend(&self) -> suspend::SuspendClient<T> {
        suspend::SuspendClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> GuestClient<T>
where
    T: crate::client::Client,
{
    pub fn migrate(&self) -> migrate::MigrateClient<T> {
        migrate::MigrateClient::<T>::new(self.client.clone(), &self.path)
    }
}
