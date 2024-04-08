pub struct VersionClient<T> {
    client: T,
    path: String,
}
impl<T> VersionClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "/version".to_string(),
        }
    }
}
impl<T> VersionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "API version details, including some parts of the global datacenter config."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutput {
    pub fn new(release: String, repoid: String, version: String) -> Self {
        Self {
            release,
            repoid,
            version,
            console: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The default console viewer to use."]
    #[doc = ""]
    pub console: Option<Console>,
    #[doc = "The current Proxmox VE point release in `x.y` format."]
    #[doc = ""]
    pub release: String,
    #[doc = "The short git revision from which this version was build."]
    #[doc = ""]
    pub repoid: String,
    #[doc = "The full pve-manager package version of this node."]
    #[doc = ""]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The default console viewer to use."]
#[doc = ""]
pub enum Console {
    #[serde(rename = "applet")]
    Applet,
    #[serde(rename = "html5")]
    Html5,
    #[serde(rename = "vv")]
    Vv,
    #[serde(rename = "xtermjs")]
    Xtermjs,
}
