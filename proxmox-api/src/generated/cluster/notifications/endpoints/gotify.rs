pub mod name;
#[derive(Debug, Clone)]
pub struct GotifyClient<T> {
    client: T,
    path: String,
}
impl<T> GotifyClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/gotify"),
        }
    }
}
impl<T> GotifyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns a list of all gotify endpoints"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> GotifyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new gotify endpoint"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        match self.client.post(&path, &params) {
            Ok(o) => Ok(o),
            Err(e) if crate::client::Error::is_empty_data(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
impl GetOutputItems {
    pub fn new(name: String, origin: Origin, server: String) -> Self {
        Self {
            name,
            origin,
            server,
            comment: Default::default(),
            disable: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
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
    #[doc = "The name of the endpoint."]
    #[doc = ""]
    pub name: String,
    #[doc = "Show if this entry was created by a user or was built-in"]
    #[doc = ""]
    pub origin: Origin,
    #[doc = "Server URL"]
    #[doc = ""]
    pub server: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(name: String, server: String, token: String) -> Self {
        Self {
            name,
            server,
            token,
            comment: Default::default(),
            disable: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
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
    #[doc = "The name of the endpoint."]
    #[doc = ""]
    pub name: String,
    #[doc = "Server URL"]
    #[doc = ""]
    pub server: String,
    #[doc = "Secret token"]
    #[doc = ""]
    pub token: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
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
impl<T> GotifyClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
