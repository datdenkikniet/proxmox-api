pub mod groups;
pub mod resources;
pub mod status;
pub struct HaClient<T> {
    client: T,
    path: String,
}
impl<T> HaClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ha"),
        }
    }
}
impl GetOutputItems {
    pub fn new(id: String) -> Self {
        Self {
            id,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub id: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> HaClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> HaClient<T>
where
    T: crate::client::Client,
{
    pub fn resources(&self) -> resources::ResourcesClient<T> {
        resources::ResourcesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> HaClient<T>
where
    T: crate::client::Client,
{
    pub fn groups(&self) -> groups::GroupsClient<T> {
        groups::GroupsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> HaClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
