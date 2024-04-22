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
impl<T> RemoteMigrateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migrate virtual machine to a remote cluster. Creates a new migration task. EXPERIMENTAL feature!"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(
        target_bridge: Vec<String>,
        target_endpoint: String,
        target_storage: Vec<String>,
    ) -> Self {
        Self {
            target_bridge,
            target_endpoint,
            target_storage,
            bwlimit: Default::default(),
            delete: Default::default(),
            online: Default::default(),
            target_vmid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Delete the original VM and related data after successful migration. By default the original VM is kept on the source cluster in a stopped state."]
    #[doc = ""]
    pub delete: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use online/live migration if VM is running. Ignored if VM is stopped."]
    #[doc = ""]
    pub online: Option<bool>,
    #[serde(rename = "target-bridge")]
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Mapping from source to target bridges. Providing only a single bridge ID maps all source bridges to that bridge. Providing the special value '1' will map each source bridge to itself."]
    #[doc = ""]
    pub target_bridge: Vec<String>,
    #[serde(rename = "target-endpoint")]
    #[doc = "Remote target endpoint"]
    #[doc = ""]
    pub target_endpoint: String,
    #[serde(rename = "target-storage")]
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Mapping from source to target storages. Providing only a single storage ID maps all source storages to that storage. Providing the special value '1' will map each source storage to itself."]
    #[doc = ""]
    pub target_storage: Vec<String>,
    #[serde(rename = "target-vmid")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub target_vmid: Option<crate::types::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
