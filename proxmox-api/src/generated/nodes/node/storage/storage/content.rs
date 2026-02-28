pub mod volume;
#[derive(Debug, Clone)]
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
    pub fn new(format: String, size: i64, volid: String) -> Self {
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Creation time (seconds since the UNIX Epoch)."]
    #[doc = ""]
    pub ctime: Option<CtimeInt>,
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
    pub size: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used space. Please note that most storage plugins do not report anything useful here."]
    #[doc = ""]
    pub used: Option<i64>,
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
    pub vmid: Option<i64>,
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
    pub vmid: Option<VmidInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(filename: String, size: SizeStr, vmid: VmidInt) -> Self {
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
    pub size: SizeStr,
    #[doc = "Specify owner VM"]
    #[doc = ""]
    pub vmid: VmidInt,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CtimeInt(i128);
impl crate::types::bounded_integer::BoundedInteger for CtimeInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for CtimeInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for CtimeInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CtimeInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VmidInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VmidInt {
    const MIN: Option<i128> = Some(100i128);
    const MAX: Option<i128> = Some(999999999i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 100 and 999999999";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VmidInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VmidInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VmidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SizeStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for SizeStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("\\d+[MG]?");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"\\d+[MG]?\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for SizeStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for SizeStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SizeStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
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
