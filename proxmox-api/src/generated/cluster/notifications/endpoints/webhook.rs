pub mod name;
#[derive(Debug, Clone)]
pub struct WebhookClient<T> {
    client: T,
    path: String,
}
impl<T> WebhookClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/webhook"),
        }
    }
}
impl<T> WebhookClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of all webhook endpoints"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> WebhookClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new webhook endpoint"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(method: Method, name: String, origin: Origin, url: String) -> Self {
        Self {
            method,
            name,
            origin,
            url,
            body: Default::default(),
            comment: Default::default(),
            disable: Default::default(),
            header: Default::default(),
            secret: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP body, base64 encoded"]
    #[doc = ""]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this target"]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "HTTP headers to set. These have to be formatted as a property string in the format name=\\\\<name\\\\>,value=\\\\<base64 of value\\\\>"]
    #[doc = ""]
    pub header: Vec<String>,
    #[doc = "HTTP method"]
    #[doc = ""]
    pub method: Method,
    #[doc = "The name of the endpoint."]
    #[doc = ""]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    #[doc = ""]
    pub origin: Origin,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Secrets to set. These have to be formatted as a property string in the format name=\\\\<name\\\\>,value=\\\\<base64 of value\\\\>"]
    #[doc = ""]
    pub secret: Vec<String>,
    #[doc = "Server URL"]
    #[doc = ""]
    pub url: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(method: Method, name: String, url: String) -> Self {
        Self {
            method,
            name,
            url,
            body: Default::default(),
            comment: Default::default(),
            disable: Default::default(),
            header: Default::default(),
            secret: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP body, base64 encoded"]
    #[doc = ""]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable this target"]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "HTTP headers to set. These have to be formatted as a property string in the format name=\\\\<name\\\\>,value=\\\\<base64 of value\\\\>"]
    #[doc = ""]
    pub header: Vec<String>,
    #[doc = "HTTP method"]
    #[doc = ""]
    pub method: Method,
    #[doc = "The name of the endpoint."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Secrets to set. These have to be formatted as a property string in the format name=\\\\<name\\\\>,value=\\\\<base64 of value\\\\>"]
    #[doc = ""]
    pub secret: Vec<String>,
    #[doc = "Server URL"]
    #[doc = ""]
    pub url: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "HTTP method"]
#[doc = ""]
pub enum Method {
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "post")]
    Post,
    #[serde(rename = "put")]
    Put,
}
impl TryFrom<&str> for Method {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "put" => Ok(Self::Put),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Show if this entry was created by a user or was built-in"]
#[doc = ""]
pub enum Origin {
    #[serde(rename = "builtin")]
    Builtin,
    #[serde(rename = "modified-builtin")]
    ModifiedBuiltin,
    #[serde(rename = "user-created")]
    UserCreated,
}
impl TryFrom<&str> for Origin {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "builtin" => Ok(Self::Builtin),
            "modified-builtin" => Ok(Self::ModifiedBuiltin),
            "user-created" => Ok(Self::UserCreated),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> WebhookClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
