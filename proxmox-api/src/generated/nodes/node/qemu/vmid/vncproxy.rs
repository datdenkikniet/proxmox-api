#[derive(Debug, Clone)]
pub struct VncproxyClient<T> {
    client: T,
    path: String,
}
impl<T> VncproxyClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/vncproxy"),
        }
    }
}
impl<T> VncproxyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Creates a TCP VNC proxy connections."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/vms/{vmid}\", [\"VM.Console\"])"]
    pub async fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params).await
    }
}
impl PostOutput {
    pub fn new(cert: String, port: i64, ticket: String, upid: String, user: String) -> Self {
        Self {
            cert,
            port,
            ticket,
            upid,
            user,
            password: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    pub cert: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Returned if requested with 'generate-password' param. Consists of printable ASCII characters ('!' .. '~')."]
    #[doc = ""]
    pub password: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub port: i64,
    pub ticket: String,
    pub upid: String,
    pub user: String,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "generate-password")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Generates a random password to be used as ticket instead of the API ticket."]
    #[doc = ""]
    pub generate_password: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prepare for websocket upgrade (only required when using serial terminal, otherwise upgrade is always possible)."]
    #[doc = ""]
    pub websocket: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
