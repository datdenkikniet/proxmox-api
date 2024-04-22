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
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List Zpools."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a ZFS pool."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
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
    pub alloc: u64,
    #[serde(
        serialize_with = "crate::types::serialize_number",
        deserialize_with = "crate::types::deserialize_number"
    )]
    pub dedup: f64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub frag: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub free: u64,
    pub health: String,
    pub name: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub size: u64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(devices: Vec<String>, name: String, raidlevel: Raidlevel) -> Self {
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
    #[doc = ""]
    pub add_storage: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pool sector size exponent."]
    #[doc = ""]
    pub ashift: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The compression algorithm to use."]
    #[doc = ""]
    pub compression: Option<Compression>,
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The block devices you want to create the zpool on."]
    #[doc = ""]
    pub devices: Vec<String>,
    #[serde(rename = "draid-config")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub draid_config: Option<String>,
    #[doc = "The storage identifier."]
    #[doc = ""]
    pub name: String,
    #[doc = "The RAID level to use."]
    #[doc = ""]
    pub raidlevel: Raidlevel,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The compression algorithm to use."]
#[doc = ""]
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
impl TryFrom<&str> for Compression {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "gzip" => Ok(Self::Gzip),
            "lz4" => Ok(Self::Lz4),
            "lzjb" => Ok(Self::Lzjb),
            "off" => Ok(Self::Off),
            "on" => Ok(Self::On),
            "zle" => Ok(Self::Zle),
            "zstd" => Ok(Self::Zstd),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Compression {
    fn default() -> Self {
        Self::On
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "The RAID level to use."]
#[doc = ""]
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
impl TryFrom<&str> for Raidlevel {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "draid" => Ok(Self::Draid),
            "draid2" => Ok(Self::Draid2),
            "draid3" => Ok(Self::Draid3),
            "mirror" => Ok(Self::Mirror),
            "raid10" => Ok(Self::Raid10),
            "raidz" => Ok(Self::Raidz),
            "raidz2" => Ok(Self::Raidz2),
            "raidz3" => Ok(Self::Raidz3),
            "single" => Ok(Self::Single),
            v => Err(format!("Unknown variant {v}")),
        }
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
