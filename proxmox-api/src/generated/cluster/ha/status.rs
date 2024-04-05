pub mod current;
pub mod manager_status;
pub struct StatusClient<T> {
    client: T,
    path: String,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/status"),
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
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn current(&self) -> current::CurrentClient<T> {
        current::CurrentClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn manager_status(&self) -> manager_status::ManagerStatusClient<T> {
        manager_status::ManagerStatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
