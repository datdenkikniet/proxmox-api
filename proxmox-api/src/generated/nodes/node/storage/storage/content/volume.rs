pub struct VolumeClient<T> {
    client: T,
    path: String,
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, volume: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, volume),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a VolumeClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete volume"]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<Option<String>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.delete(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<DeleteParams, Option<String>, T::Error>
    for &VolumeClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Delete;
    fn exec(&self, params: DeleteParams) -> Result<Option<String>, T::Error> {
        self.delete(params)
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get volume attributes"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), GetOutput, T::Error> for &VolumeClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<GetOutput, T::Error> {
        self.get()
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Copy a volume. This is experimental code - do not use."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, String, T::Error>
    for &VolumeClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<String, T::Error> {
        self.post(params)
    }
}
impl<T> VolumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update volume attributes"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PutParams, (), T::Error> for &VolumeClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Put;
    fn exec(&self, params: PutParams) -> Result<(), T::Error> {
        self.put(params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Time to wait for the task to finish. We return 'null' if the task finish within that time."]
    #[doc = ""]
    pub delay: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(format: String, path: String, size: u64, used: u64) -> Self {
        Self {
            format,
            path,
            size,
            used,
            notes: Default::default(),
            protected: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[doc = "Format identifier ('raw', 'qcow2', 'subvol', 'iso', 'tgz' ...)"]
    #[doc = ""]
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Optional notes."]
    #[doc = ""]
    pub notes: Option<String>,
    #[doc = "The Path"]
    #[doc = ""]
    pub path: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Protection status. Currently only supported for backups."]
    #[doc = ""]
    pub protected: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Volume size in bytes."]
    #[doc = ""]
    pub size: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Used space. Please note that most storage plugins do not report anything useful here."]
    #[doc = ""]
    pub used: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(target: String) -> Self {
        Self {
            target,
            target_node: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Target volume identifier"]
    #[doc = ""]
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Target node. Default is local node."]
    #[doc = ""]
    pub target_node: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The new notes."]
    #[doc = ""]
    pub notes: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Protection status. Currently only supported for backups."]
    #[doc = ""]
    pub protected: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
