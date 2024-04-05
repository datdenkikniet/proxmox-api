pub mod name;
pub struct DirectoryClient<T> {
    client: T,
    path: String,
}
impl<T> DirectoryClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/directory"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Filesystem {
    #[serde(rename = "ext4")]
    Ext4,
    #[serde(rename = "xfs")]
    Xfs,
}
impl Default for Filesystem {
    fn default() -> Self {
        Self::Ext4
    }
}
impl GetOutputItems {
    pub fn new(
        device: String,
        options: String,
        path: String,
        ty: String,
        unitfile: String,
    ) -> Self {
        Self {
            device,
            options,
            path,
            ty,
            unitfile,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The mounted device."]
    pub device: String,
    #[doc = "The mount options."]
    pub options: String,
    #[doc = "The mount path."]
    pub path: String,
    #[serde(rename = "type")]
    #[doc = "The filesystem type."]
    pub ty: String,
    #[doc = "The path of the mount unit."]
    pub unitfile: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DirectoryClient<T>
where
    T: crate::client::Client,
{
    #[doc = "PVE Managed Directory storages."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(device: String, name: String) -> Self {
        Self {
            device,
            name,
            add_storage: Default::default(),
            filesystem: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure storage using the directory."]
    pub add_storage: Option<bool>,
    #[doc = "The block device you want to create the filesystem on."]
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The desired filesystem."]
    pub filesystem: Option<Filesystem>,
    #[doc = "The storage identifier."]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DirectoryClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a Filesystem on an unused disk. Will be mounted under '/mnt/pve/NAME'."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> DirectoryClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
