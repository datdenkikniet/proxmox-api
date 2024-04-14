pub mod volume;
pub struct ContentClient<T> {
    client: T,
    path: String,
}
impl<T> ContentClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/content"),
        }
    }
}
impl<T> ContentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List storage content."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ContentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Allocate disk images."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(format: String, size: u64, volid: String) -> Self {
        Self {
            format,
            size,
            volid,
            ctime: Default::default(),
            encrypted: Default::default(),
            notes: Default::default(),
            parent: Default::default(),
            protected: Default::default(),
            used: Default::default(),
            verification: Default::default(),
            vmid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Creation time (seconds since the UNIX Epoch)."]
    #[doc = ""]
    pub ctime: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If whole backup is encrypted, value is the fingerprint or '1'  if encrypted. Only useful for the Proxmox Backup Server storage type."]
    #[doc = ""]
    pub encrypted: Option<String>,
    #[doc = "Format identifier ('raw', 'qcow2', 'subvol', 'iso', 'tgz' ...)"]
    #[doc = ""]
    pub format: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Optional notes. If they contain multiple lines, only the first one is returned here."]
    #[doc = ""]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Volume identifier of parent (for linked cloned)."]
    #[doc = ""]
    pub parent: Option<String>,
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
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used space. Please note that most storage plugins do not report anything useful here."]
    #[doc = ""]
    pub used: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Last backup verification result, only useful for PBS storages."]
    #[doc = ""]
    pub verification: Option<VerificationGetOutputItemsVerification>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Associated Owner VMID."]
    #[doc = ""]
    pub vmid: Option<u64>,
    #[doc = "Volume identifier."]
    #[doc = ""]
    pub volid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list content of this type."]
    #[doc = ""]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list images for this VM"]
    #[doc = ""]
    pub vmid: Option<crate::types::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(filename: String, size: String, vmid: crate::types::VmId) -> Self {
        Self {
            filename,
            size,
            vmid,
            format: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "The name of the file to create."]
    #[doc = ""]
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub format: Option<Format>,
    #[doc = "Size in kilobyte (1024 bytes). Optional suffixes 'M' (megabyte, 1024K) and 'G' (gigabyte, 1024M)"]
    #[doc = ""]
    pub size: String,
    #[doc = "Specify owner VM"]
    #[doc = ""]
    pub vmid: crate::types::VmId,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl VerificationGetOutputItemsVerification {
    pub fn new(state: String, upid: String) -> Self {
        Self {
            state,
            upid,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct VerificationGetOutputItemsVerification {
    #[doc = "Last backup verification state."]
    #[doc = ""]
    pub state: String,
    #[doc = "Last backup verification UPID."]
    #[doc = ""]
    pub upid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Format {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "subvol")]
    Subvol,
}
impl TryFrom<&str> for Format {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "qcow2" => Ok(Self::Qcow2),
            "raw" => Ok(Self::Raw),
            "subvol" => Ok(Self::Subvol),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> ContentClient<T>
where
    T: crate::client::Client,
{
    pub fn volume(&self, volume: &str) -> volume::VolumeClient<T> {
        volume::VolumeClient::<T>::new(self.client.clone(), &self.path, volume)
    }
}
