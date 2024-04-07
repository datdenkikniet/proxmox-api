pub struct InitgptClient<T> {
    client: T,
    path: String,
}
impl<T> InitgptClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/initgpt"),
        }
    }
}
impl<T> InitgptClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Initialize Disk with GPT"]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(disk: String) -> Self {
        Self {
            disk,
            uuid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Block device name"]
    pub disk: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "UUID for the GPT table"]
    pub uuid: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
