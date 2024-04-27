#[derive(Debug, Clone)]
pub struct GetVcpusClient<T> {
    client: T,
    path: String,
}
impl<T> GetVcpusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/get-vcpus"),
        }
    }
}
impl<T> GetVcpusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Execute get-vcpus."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
