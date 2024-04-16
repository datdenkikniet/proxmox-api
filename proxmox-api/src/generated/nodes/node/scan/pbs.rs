pub struct PbsClient<T> {
    client: T,
    path: String,
}
impl<T> PbsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/pbs"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a PbsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> PbsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Scan remote Proxmox Backup Server."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<GetParams, Vec<GetOutputItems>, T::Error>
    for &PbsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get(params)
    }
}
impl GetOutputItems {
    pub fn new(store: String) -> Self {
        Self {
            store,
            comment: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comment from server."]
    #[doc = ""]
    pub comment: Option<String>,
    #[doc = "The datastore name."]
    #[doc = ""]
    pub store: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(password: String, server: String, username: String) -> Self {
        Self {
            password,
            server,
            username,
            fingerprint: Default::default(),
            port: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    #[doc = ""]
    pub fingerprint: Option<String>,
    #[doc = "User password or API token secret."]
    #[doc = ""]
    pub password: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Optional port."]
    #[doc = ""]
    pub port: Option<u64>,
    #[doc = "The server address (name or IP)."]
    #[doc = ""]
    pub server: String,
    #[doc = "User-name or API token-ID."]
    #[doc = ""]
    pub username: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
