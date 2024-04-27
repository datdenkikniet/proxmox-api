#[derive(Debug, Clone)]
pub struct ExecuteClient<T> {
    client: T,
    path: String,
}
impl<T> ExecuteClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/execute"),
        }
    }
}
impl<T> ExecuteClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Execute multiple commands in order, root only."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<Vec<PostOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(commands: String) -> Self {
        Self {
            commands,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "JSON encoded array of commands."]
    #[doc = ""]
    #[doc = "JSON encoded array of commands, where each command is an object with the following properties:"]
    #[doc = ""]
    #[doc = "args:      \\\\<object\\\\>"]
    #[doc = ""]
    #[doc = "A set of parameter names and their values."]
    #[doc = ""]
    #[doc = "method:    (GET|POST|PUT|DELETE)"]
    #[doc = ""]
    #[doc = "A method related to the API endpoint (GET, POST etc.)."]
    #[doc = ""]
    #[doc = "path:      \\\\<string\\\\>"]
    #[doc = ""]
    #[doc = "A relative path to an API endpoint on this node."]
    #[doc = ""]
    pub commands: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
