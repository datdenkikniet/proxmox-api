#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Remove webhook endpoint"]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Return a specific webhook endpoint"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update existing webhook endpoint"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl GetOutput {
    pub fn new(method: Method, name: String, url: String) -> Self {
        Self {
            method,
            name,
            url,
            body: Default::default(),
            comment: Default::default(),
            digest: Default::default(),
            disable: Default::default(),
            header: Default::default(),
            secret: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP body, base64 encoded"]
    #[doc = ""]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP body, base64 encoded"]
    #[doc = ""]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment"]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HTTP method"]
    #[doc = ""]
    pub method: Option<Method>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Secrets to set. These have to be formatted as a property string in the format name=\\\\<name\\\\>,value=\\\\<base64 of value\\\\>"]
    #[doc = ""]
    pub secret: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Server URL"]
    #[doc = ""]
    pub url: Option<String>,
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
