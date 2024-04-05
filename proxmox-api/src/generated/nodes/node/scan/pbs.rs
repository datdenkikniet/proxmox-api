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
    pub fingerprint: Option<String>,
    #[doc = "User password or API token secret."]
    pub password: String,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Optional port."]
    pub port: Option<u64>,
    #[doc = "The server address (name or IP)."]
    pub server: String,
    #[doc = "User-name or API token-ID."]
    pub username: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
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
    pub comment: Option<String>,
    #[doc = "The datastore name."]
    pub store: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> PbsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Scan remote Proxmox Backup Server."]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
