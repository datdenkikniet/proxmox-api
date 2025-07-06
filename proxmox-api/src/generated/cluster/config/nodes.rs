pub mod node;
#[derive(Debug, Clone)]
pub struct NodesClient<T> {
    client: T,
    path: String,
}
impl<T> NodesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/nodes"),
        }
    }
}
impl<T> NodesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Corosync node list."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(node: String) -> Self {
        Self {
            node,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub node: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> NodesClient<T>
where
    T: crate::client::Client,
{
    pub fn node(&self, node: &str) -> node::NodeClient<T> {
        node::NodeClient::<T>::new(self.client.clone(), &self.path, node)
    }
}
