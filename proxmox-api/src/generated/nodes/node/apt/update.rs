#[derive(Debug, Clone)]
pub struct UpdateClient<T> {
    client: T,
    path: String,
}
impl<T> UpdateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/update"),
        }
    }
}
impl<T> UpdateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List available updates."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> UpdateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "This is used to resynchronize the package index files from their sources (apt-get update)."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(
        arch: Arch,
        description: String,
        origin: String,
        package: String,
        priority: String,
        section: String,
        title: String,
        version: String,
    ) -> Self {
        Self {
            arch,
            description,
            origin,
            package,
            priority,
            section,
            title,
            version,
            notifystatus: ::std::default::Default::default(),
            oldversion: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(rename = "Arch")]
    #[doc = "Package Architecture."]
    #[doc = ""]
    pub arch: Arch,
    #[serde(rename = "Description")]
    #[doc = "Package description."]
    #[doc = ""]
    pub description: String,
    #[serde(rename = "NotifyStatus")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Version for which PVE has already sent an update notification for."]
    #[doc = ""]
    pub notifystatus: Option<String>,
    #[serde(rename = "OldVersion")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Old version currently installed."]
    #[doc = ""]
    pub oldversion: Option<String>,
    #[serde(rename = "Origin")]
    #[doc = "Package origin, e.g., 'Proxmox' or 'Debian'."]
    #[doc = ""]
    pub origin: String,
    #[serde(rename = "Package")]
    #[doc = "Package name."]
    #[doc = ""]
    pub package: String,
    #[serde(rename = "Priority")]
    #[doc = "Package priority."]
    #[doc = ""]
    pub priority: String,
    #[serde(rename = "Section")]
    #[doc = "Package section."]
    #[doc = ""]
    pub section: String,
    #[serde(rename = "Title")]
    #[doc = "Package title."]
    #[doc = ""]
    pub title: String,
    #[serde(rename = "Version")]
    #[doc = "New version to be updated to."]
    #[doc = ""]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Send notification about new packages."]
    #[doc = ""]
    pub notify: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only produces output suitable for logging, omitting progress indicators."]
    #[doc = ""]
    pub quiet: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Package Architecture."]
#[doc = ""]
pub enum Arch {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "amd64")]
    Amd64,
    #[serde(rename = "arm64")]
    Arm64,
    #[serde(rename = "armhf")]
    Armhf,
    #[serde(rename = "ppc64el")]
    Ppc64el,
    #[serde(rename = "risc64")]
    Risc64,
    #[serde(rename = "s390x")]
    S390x,
}
impl TryFrom<&str> for Arch {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "all" => Ok(Self::All),
            "amd64" => Ok(Self::Amd64),
            "arm64" => Ok(Self::Arm64),
            "armhf" => Ok(Self::Armhf),
            "ppc64el" => Ok(Self::Ppc64el),
            "risc64" => Ok(Self::Risc64),
            "s390x" => Ok(Self::S390x),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
