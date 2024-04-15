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
impl<'a, T> crate::ProxmoxClient for &'a DirectoryClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> DirectoryClient<T>
where
    T: crate::client::Client,
{
    #[doc = "PVE Managed Directory storages."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> DirectoryClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a Filesystem on an unused disk. Will be mounted under '/mnt/pve/NAME'."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
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
    #[doc = ""]
    pub device: String,
    #[doc = "The mount options."]
    #[doc = ""]
    pub options: String,
    #[doc = "The mount path."]
    #[doc = ""]
    pub path: String,
    #[serde(rename = "type")]
    #[doc = "The filesystem type."]
    #[doc = ""]
    pub ty: String,
    #[doc = "The path of the mount unit."]
    #[doc = ""]
    pub unitfile: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
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
    #[doc = ""]
    pub add_storage: Option<bool>,
    #[doc = "The block device you want to create the filesystem on."]
    #[doc = ""]
    pub device: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The desired filesystem."]
    #[doc = ""]
    pub filesystem: Option<Filesystem>,
    #[doc = "The storage identifier."]
    #[doc = ""]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The desired filesystem."]
#[doc = ""]
pub enum Filesystem {
    #[serde(rename = "ext4")]
    Ext4,
    #[serde(rename = "xfs")]
    Xfs,
}
impl TryFrom<&str> for Filesystem {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ext4" => Ok(Self::Ext4),
            "xfs" => Ok(Self::Xfs),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Filesystem {
    fn default() -> Self {
        Self::Ext4
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
