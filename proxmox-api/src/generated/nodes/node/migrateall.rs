pub struct MigrateallClient<T> {
    client: T,
    path: String,
}
impl<T> MigrateallClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/migrateall"),
        }
    }
}
impl PostParams {
    pub fn new(target: String) -> Self {
        Self {
            target,
            maxworkers: Default::default(),
            vms: Default::default(),
            with_local_disks: Default::default(),
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
    #[doc = "Maximal number of parallel migration job. If not set, uses'max_workers' from datacenter.cfg. One of both must be set!"]
    pub maxworkers: Option<u64>,
    #[doc = "Target node."]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only consider Guests with these IDs."]
    pub vms: Option<String>,
    #[serde(rename = "with-local-disks")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable live storage migration for local disk"]
    pub with_local_disks: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MigrateallClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migrate all VMs and Containers."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
