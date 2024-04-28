#[derive(Debug, Clone)]
pub struct MigrateClient<T> {
    client: T,
    path: String,
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/migrate"),
        }
    }
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Request resource migration (online) to another node."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        match self.client.post(&path, &params) {
            Ok(o) => Ok(o),
            Err(e) if crate::client::Error::is_empty_data(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
impl PostParams {
    pub fn new(node: String) -> Self {
        Self {
            node,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Target node."]
    #[doc = ""]
    pub node: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
