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
    #[doc = "Permission check: perm(\"/\", [\"Sys.Audit\"])"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(node: String) -> Self {
        Self {
            node,
            additional_properties: ::std::default::Default::default(),
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
