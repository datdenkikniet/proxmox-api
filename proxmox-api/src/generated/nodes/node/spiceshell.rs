pub struct SpiceshellClient<T> {
    client: T,
    path: String,
}
impl<T> SpiceshellClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/spiceshell"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a SpiceshellClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> SpiceshellClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Creates a SPICE shell."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "SPICE proxy server. This can be used by the client to specify the proxy server. All nodes in a cluster runs 'spiceproxy', so it is up to the client to choose one. By default, we return the node where the VM is currently running. As reasonable setting is to use same node you use to connect to the API (This is window.location.hostname for the JS GUI)."]
    #[doc = ""]
    pub proxy: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Run specific command or default to login (requires 'root@pam')"]
#[doc = ""]
pub enum Cmd {
    #[serde(rename = "ceph_install")]
    CephInstall,
    #[serde(rename = "login")]
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
impl Default for Cmd {
    fn default() -> Self {
        Self::Login
    }
}
