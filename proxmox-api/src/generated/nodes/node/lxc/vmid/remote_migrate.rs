pub struct RemoteMigrateClient<T> {
    client: T,
    path: String,
}
impl<T> RemoteMigrateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/remote_migrate"),
        }
    }
}
impl PostParams {
    pub fn new(target_bridge: String, target_endpoint: String, target_storage: String) -> Self {
        Self {
            target_bridge,
            target_endpoint,
            target_storage,
            bwlimit: Default::default(),
            delete: Default::default(),
            online: Default::default(),
            restart: Default::default(),
            target_vmid: Default::default(),
            timeout: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    pub bwlimit: Option<()>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Delete the original CT and related data after successful migration. By default the original CT is kept on the source cluster in a stopped state."]
    pub delete: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use online/live migration."]
    pub online: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use restart migration"]
    pub restart: Option<bool>,
    #[serde(rename = "target-bridge")]
    #[doc = "Mapping from source to target bridges. Providing only a single bridge ID maps all source bridges to that bridge. Providing the special value '1' will map each source bridge to itself."]
    pub target_bridge: String,
    #[serde(rename = "target-endpoint")]
    #[doc = "Remote target endpoint"]
    pub target_endpoint: String,
    #[serde(rename = "target-storage")]
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    pub target_storage: String,
    #[serde(rename = "target-vmid")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The (unique) ID of the VM."]
    pub target_vmid: Option<crate::types::VmId>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Timeout in seconds for shutdown for restart migration"]
    pub timeout: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> RemoteMigrateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migrate the container to another cluster. Creates a new migration task. EXPERIMENTAL feature!"]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
