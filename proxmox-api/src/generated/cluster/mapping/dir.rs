pub mod id;
#[derive(Debug, Clone)]
pub struct DirClient<T> {
    client: T,
    path: String,
}
impl<T> DirClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/dir"),
        }
    }
}
impl<T> DirClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List directory mapping"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> DirClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new directory mapping."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl ChecksGetOutputItemsChecksItems {
    pub fn new(message: String, severity: Severity) -> Self {
        Self {
            message,
            severity,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct ChecksGetOutputItemsChecksItems {
    #[doc = "The message of the error"]
    #[doc = ""]
    pub message: String,
    #[doc = "The severity of the error"]
    #[doc = ""]
    pub severity: Severity,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(description: String, id: String, map: Vec<String>) -> Self {
        Self {
            description,
            id,
            map,
            checks: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of checks, only present if 'check-node' is set."]
    #[doc = ""]
    pub checks: Vec<ChecksGetOutputItemsChecksItems>,
    #[doc = "A description of the logical mapping."]
    #[doc = ""]
    pub description: String,
    #[doc = "The logical ID of the mapping."]
    #[doc = ""]
    pub id: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The entries of the mapping."]
    #[doc = ""]
    pub map: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "check-node")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If given, checks the configurations on the given node for correctness, and adds relevant diagnostics for the directory to the response."]
    #[doc = ""]
    pub check_node: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(id: String, map: Vec<String>) -> Self {
        Self {
            id,
            map,
            description: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description of the directory mapping"]
    #[doc = ""]
    pub description: Option<String>,
    #[doc = "The ID of the directory mapping"]
    #[doc = ""]
    pub id: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of maps for the cluster nodes."]
    #[doc = ""]
    pub map: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The severity of the error"]
#[doc = ""]
pub enum Severity {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
}
impl TryFrom<&str> for Severity {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> DirClient<T>
where
    T: crate::client::Client,
{
    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
