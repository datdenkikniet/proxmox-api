pub mod all;
pub mod fabric;
pub mod node;
#[derive(Debug, Clone)]
pub struct FabricsClient<T> {
    client: T,
    path: String,
}
impl<T> FabricsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/fabrics"),
        }
    }
}
impl<T> FabricsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN Fabrics Index"]
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
impl<T> FabricsClient<T>
where
    T: crate::client::Client,
{
    pub fn fabric(&self) -> fabric::FabricClient<T> {
        fabric::FabricClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FabricsClient<T>
where
    T: crate::client::Client,
{
    pub fn node(&self) -> node::NodeClient<T> {
        node::NodeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FabricsClient<T>
where
    T: crate::client::Client,
{
    pub fn all(&self) -> all::AllClient<T> {
        all::AllClient::<T>::new(self.client.clone(), &self.path)
    }
}
