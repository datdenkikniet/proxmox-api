#[derive(Debug, Clone)]
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
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostOutput {
    pub fn new(pid: i64) -> Self {
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
    pub pid: i64,
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
    pub input_data: Option<InputDataStr>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InputDataStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for InputDataStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(65536usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 65536";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for InputDataStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for InputDataStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for InputDataStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
