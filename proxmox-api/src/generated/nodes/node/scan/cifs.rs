pub struct CifsClient<T> {
    client: T,
    path: String,
}
impl<T> CifsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/cifs"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a CifsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> CifsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Scan remote CIFS server."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(description: String, share: String) -> Self {
        Self {
            description,
            share,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Descriptive text from server."]
    #[doc = ""]
    pub description: String,
    #[doc = "The cifs share name."]
    #[doc = ""]
    pub share: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(server: String) -> Self {
        Self {
            server,
            domain: Default::default(),
            password: Default::default(),
            username: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "SMB domain (Workgroup)."]
    #[doc = ""]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "User password."]
    #[doc = ""]
    pub password: Option<String>,
    #[doc = "The server address (name or IP)."]
    #[doc = ""]
    pub server: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "User name."]
    #[doc = ""]
    pub username: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
