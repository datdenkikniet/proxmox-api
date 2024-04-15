pub struct StartClient<T> {
    client: T,
    path: String,
}
impl<T> StartClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/start"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a StartClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> StartClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Start virtual machine."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "force-cpu")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override QEMU's -cpu argument with the given string."]
    #[doc = ""]
    pub force_cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies the QEMU machine type."]
    #[doc = ""]
    pub machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name."]
    #[doc = ""]
    pub migratedfrom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CIDR of the (sub) network that is used for migration."]
    #[doc = ""]
    pub migration_network: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance."]
    #[doc = ""]
    pub migration_type: Option<MigrationType>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[doc = ""]
    pub skiplock: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Some command save/restore state from this location."]
    #[doc = ""]
    pub stateuri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[doc = ""]
    pub targetstorage: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Wait maximal timeout seconds."]
    #[doc = ""]
    pub timeout: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Migration traffic is encrypted using an SSH tunnel by default. On secure, completely private networks this can be disabled to increase performance."]
#[doc = ""]
pub enum MigrationType {
    #[serde(rename = "insecure")]
    Insecure,
    #[serde(rename = "secure")]
    Secure,
}
impl TryFrom<&str> for MigrationType {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "insecure" => Ok(Self::Insecure),
            "secure" => Ok(Self::Secure),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
