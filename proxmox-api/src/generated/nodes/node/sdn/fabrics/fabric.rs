pub mod interfaces;
pub mod neighbors;
pub mod routes;
#[derive(Debug, Clone)]
pub struct FabricClient<T> {
    client: T,
    path: String,
}
impl<T> FabricClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, fabric: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, fabric),
        }
    }
}
impl<T> FabricClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index for SDN fabric status."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(subdir: String) -> Self {
        Self {
            subdir,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub subdir: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> FabricClient<T>
where
    T: crate::client::Client,
{
    pub fn routes(&self) -> routes::RoutesClient<T> {
        routes::RoutesClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FabricClient<T>
where
    T: crate::client::Client,
{
    pub fn neighbors(&self) -> neighbors::NeighborsClient<T> {
        neighbors::NeighborsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FabricClient<T>
where
    T: crate::client::Client,
{
    pub fn interfaces(&self) -> interfaces::InterfacesClient<T> {
        interfaces::InterfacesClient::<T>::new(self.client.clone(), &self.path)
    }
}
