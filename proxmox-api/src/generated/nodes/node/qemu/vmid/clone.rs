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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Format {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "vmdk")]
    Vmdk,
}
impl PostParams {
    pub fn new(newid: crate::VmId) -> Self {
        Self {
            newid,
            bwlimit: Default::default(),
            description: Default::default(),
            format: Default::default(),
            full: Default::default(),
            name: Default::default(),
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    pub bwlimit: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the new VM."]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target format for file storage. Only valid for full clone."]
    pub format: Option<Format>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create a full copy of all disks. This is always done when you clone a normal VM. For VM templates, we try to create a linked clone by default."]
    pub full: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set a name for the new VM."]
    pub name: Option<String>,
    #[doc = "VMID for the clone."]
    pub newid: crate::VmId,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add the new VM to the specified pool."]
    pub pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The name of the snapshot."]
    pub snapname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target storage for full clone."]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target node. Only allowed if the original VM is on shared storage."]
    pub target: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> CloneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a copy of virtual machine/template."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
