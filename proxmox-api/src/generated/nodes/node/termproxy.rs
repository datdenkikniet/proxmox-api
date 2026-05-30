#[derive(Debug, Clone)]
pub struct TermproxyClient<T> {
    client: T,
    path: String,
}
impl<T> TermproxyClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/termproxy"),
        }
    }
}
impl<T> TermproxyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Creates a VNC Shell proxy."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/nodes/{node}\", [\"Sys.Console\"])"]
    pub async fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params).await
    }
}
impl PostOutput {
    pub fn new(port: i64, ticket: String, upid: String, user: String) -> Self {
        Self {
            port,
            ticket,
            upid,
            user,
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "port used to bind termproxy to."]
    #[doc = ""]
    pub port: i64,
    #[doc = "VNC ticket used to verify websocket connection."]
    #[doc = ""]
    pub ticket: String,
    #[doc = "UPID for termproxy worker task."]
    #[doc = ""]
    pub upid: String,
    #[doc = "user/token that generated the VNC ticket in `ticket`."]
    #[doc = ""]
    pub user: String,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Run specific command or default to login (requires 'root@pam')"]
    #[doc = ""]
    pub cmd: Option<Cmd>,
    #[serde(rename = "cmd-opts")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add parameters to a command. Encoded as null terminated strings."]
    #[doc = ""]
    pub cmd_opts: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Run specific command or default to login (requires 'root@pam')"]
#[doc = ""]
pub enum Cmd {
    #[serde(rename = "ceph_install")]
    CephInstall,
    #[serde(rename = "login")]
    #[default]
    Login,
    #[serde(rename = "upgrade")]
    Upgrade,
}
impl TryFrom<&str> for Cmd {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ceph_install" => Ok(Self::CephInstall),
            "login" => Ok(Self::Login),
            "upgrade" => Ok(Self::Upgrade),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
