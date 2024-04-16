pub struct FileWriteClient<T> {
    client: T,
    path: String,
}
impl<T> FileWriteClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/file-write"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a FileWriteClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> FileWriteClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Writes the given file via guest agent."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, (), T::Error> for &FileWriteClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<(), T::Error> {
        self.post(params)
    }
}
impl PostParams {
    pub fn new(content: String, file: String) -> Self {
        Self {
            content,
            file,
            encode: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "The content to write into the file."]
    #[doc = ""]
    pub content: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If set, the content will be encoded as base64 (required by QEMU).Otherwise the content needs to be encoded beforehand - defaults to true."]
    #[doc = ""]
    pub encode: Option<bool>,
    #[doc = "The path to the file."]
    #[doc = ""]
    pub file: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
