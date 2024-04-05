pub mod name;
pub struct ZfsClient<T> {
    client: T,
    path: String,
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/zfs"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Compression {
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "lz4")]
    Lz4,
    #[serde(rename = "lzjb")]
    Lzjb,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "on")]
    On,
    #[serde(rename = "zle")]
    Zle,
    #[serde(rename = "zstd")]
    Zstd,
}
impl Default for Compression {
    fn default() -> Self {
        Self::On
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Raidlevel {
    #[serde(rename = "draid")]
    Draid,
    #[serde(rename = "draid2")]
    Draid2,
    #[serde(rename = "draid3")]
    Draid3,
    #[serde(rename = "mirror")]
    Mirror,
    #[serde(rename = "raid10")]
    Raid10,
    #[serde(rename = "raidz")]
    Raidz,
    #[serde(rename = "raidz2")]
    Raidz2,
    #[serde(rename = "raidz3")]
    Raidz3,
    #[serde(rename = "single")]
    Single,
}
impl GetOutputItems {
    pub fn new(
        alloc: u64,
        dedup: f64,
        frag: u64,
        free: u64,
        health: String,
        name: String,
        size: u64,
    ) -> Self {
        Self {
            alloc,
            dedup,
            frag,
            free,
            health,
            name,
            size,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = ""]
    pub alloc: u64,
    #[serde(
        serialize_with = "crate::types::serialize_number",
        deserialize_with = "crate::types::deserialize_number"
    )]
    #[doc = ""]
    pub dedup: f64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = ""]
    pub frag: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = ""]
    pub free: u64,
    #[doc = ""]
    pub health: String,
    #[doc = ""]
    pub name: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = ""]
    pub size: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List Zpools."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(devices: String, name: String, raidlevel: Raidlevel) -> Self {
        Self {
            devices,
            name,
            raidlevel,
            add_storage: Default::default(),
            ashift: Default::default(),
            compression: Default::default(),
            draid_config: Default::default(),
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
    #[doc = "Configure storage using the zpool."]
    pub add_storage: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pool sector size exponent."]
    pub ashift: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The compression algorithm to use."]
    pub compression: Option<Compression>,
    #[doc = "The block devices you want to create the zpool on."]
    pub devices: String,
    #[serde(rename = "draid-config")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub draid_config: Option<String>,
    #[doc = "The storage identifier."]
    pub name: String,
    #[doc = "The RAID level to use."]
    pub raidlevel: Raidlevel,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a ZFS pool."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
