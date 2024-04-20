pub struct ExecClient<T> {
    client: T,
    path: String,
}
impl<T> ExecClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/exec"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ExecClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ExecClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Executes the given command in the vm via the guest-agent and returns an object with the pid."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, PostOutput, T::Error>
    for &ExecClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        self.post(params)
    }
}
impl PostOutput {
    pub fn new(pid: u64) -> Self {
        Self {
            pid,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The PID of the process started by the guest-agent."]
    #[doc = ""]
    pub pid: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(command: Vec<String>) -> Self {
        Self {
            command,
            input_data: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The command as a list of program + arguments."]
    #[doc = ""]
    pub command: Vec<String>,
    #[serde(rename = "input-data")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Data to pass as 'input-data' to the guest. Usually treated as STDIN to 'command'."]
    #[doc = ""]
    pub input_data: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
