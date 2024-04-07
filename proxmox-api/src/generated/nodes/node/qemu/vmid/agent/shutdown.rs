pub struct ShutdownClient<T> {
    client: T,
    path: String,
}
impl<T> ShutdownClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/shutdown"),
        }
    }
}
impl<T> ShutdownClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Execute shutdown."]
    pub fn post(&self) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
