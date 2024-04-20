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
impl<'a, T> crate::ProxmoxClient for &'a InitgptClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> InitgptClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Initialize Disk with GPT"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, String, T::Error>
    for &InitgptClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<String, T::Error> {
        self.post(params)
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
    #[doc = ""]
    pub disk: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "UUID for the GPT table"]
    #[doc = ""]
    pub uuid: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
