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
impl<T> ExecClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Executes the given command in the vm via the guest-agent and returns an object with the pid."]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
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
    pub pid: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(command: Vec<()>) -> Self {
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
    pub command: Vec<()>,
    #[serde(rename = "input-data")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Data to pass as 'input-data' to the guest. Usually treated as STDIN to 'command'."]
    pub input_data: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
