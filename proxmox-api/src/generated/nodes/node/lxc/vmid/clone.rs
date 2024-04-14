pub struct CloneClient<T> {
    client: T,
    path: String,
}
impl<T> CloneClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/clone"),
        }
    }
}
impl<T> CloneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a container clone/copy"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(newid: crate::types::VmId) -> Self {
        Self {
            newid,
            bwlimit: Default::default(),
            description: Default::default(),
            full: Default::default(),
            hostname: Default::default(),
            pool: Default::default(),
            snapname: Default::default(),
            storage: Default::default(),
            target: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the new CT."]
    #[doc = ""]
    pub description: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create a full copy of all disks. This is always done when you clone a normal CT. For CT templates, we try to create a linked clone by default."]
    #[doc = ""]
    pub full: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set a hostname for the new CT."]
    #[doc = ""]
    pub hostname: Option<String>,
    #[doc = "VMID for the clone."]
    #[doc = ""]
    pub newid: crate::types::VmId,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add the new CT to the specified pool."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The name of the snapshot."]
    #[doc = ""]
    pub snapname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target storage for full clone."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target node. Only allowed if the original VM is on shared storage."]
    #[doc = ""]
    pub target: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
