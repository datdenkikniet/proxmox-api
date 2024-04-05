pub struct RollbackClient<T> {
    client: T,
    path: String,
}
impl<T> RollbackClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/rollback"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Whether the container should get started after rolling back successfully"]
    pub start: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> RollbackClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Rollback LXC state to specified snapshot."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
