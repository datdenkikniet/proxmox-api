pub mod flag;
pub struct FlagsClient<T> {
    client: T,
    path: String,
}
impl<T> FlagsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/flags"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a FlagsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> FlagsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "get the status of all ceph flags"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &FlagsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
}
impl<T> FlagsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set/Unset multiple ceph flags at once."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PutParams, String, T::Error> for &FlagsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Put;
    fn exec(&self, params: PutParams) -> Result<String, T::Error> {
        self.put(params)
    }
}
impl GetOutputItems {
    pub fn new(description: String, name: Name, value: bool) -> Self {
        Self {
            description,
            name,
            value,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Flag description."]
    #[doc = ""]
    pub description: String,
    #[doc = "Flag name."]
    #[doc = ""]
    pub name: Name,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Flag value."]
    #[doc = ""]
    pub value: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backfilling of PGs is suspended."]
    #[doc = ""]
    pub nobackfill: Option<bool>,
    #[serde(rename = "nodeep-scrub")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deep Scrubbing is disabled."]
    #[doc = ""]
    pub nodeep_scrub: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSD failure reports are being ignored, such that the monitors will not mark OSDs down."]
    #[doc = ""]
    pub nodown: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSDs that were previously marked out will not be marked back in when they start."]
    #[doc = ""]
    pub noin: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSDs will not automatically be marked out after the configured interval."]
    #[doc = ""]
    pub noout: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Rebalancing of PGs is suspended."]
    #[doc = ""]
    pub norebalance: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Recovery of PGs is suspended."]
    #[doc = ""]
    pub norecover: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Scrubbing is disabled."]
    #[doc = ""]
    pub noscrub: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Cache tiering activity is suspended."]
    #[doc = ""]
    pub notieragent: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSDs are not allowed to start."]
    #[doc = ""]
    pub noup: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pauses read and writes."]
    #[doc = ""]
    pub pause: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Flag name."]
#[doc = ""]
pub enum Name {
    #[serde(rename = "nobackfill")]
    Nobackfill,
    #[serde(rename = "nodeep-scrub")]
    NodeepScrub,
    #[serde(rename = "nodown")]
    Nodown,
    #[serde(rename = "noin")]
    Noin,
    #[serde(rename = "noout")]
    Noout,
    #[serde(rename = "norebalance")]
    Norebalance,
    #[serde(rename = "norecover")]
    Norecover,
    #[serde(rename = "noscrub")]
    Noscrub,
    #[serde(rename = "notieragent")]
    Notieragent,
    #[serde(rename = "noup")]
    Noup,
    #[serde(rename = "pause")]
    Pause,
}
impl TryFrom<&str> for Name {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "nobackfill" => Ok(Self::Nobackfill),
            "nodeep-scrub" => Ok(Self::NodeepScrub),
            "nodown" => Ok(Self::Nodown),
            "noin" => Ok(Self::Noin),
            "noout" => Ok(Self::Noout),
            "norebalance" => Ok(Self::Norebalance),
            "norecover" => Ok(Self::Norecover),
            "noscrub" => Ok(Self::Noscrub),
            "notieragent" => Ok(Self::Notieragent),
            "noup" => Ok(Self::Noup),
            "pause" => Ok(Self::Pause),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> FlagsClient<T>
where
    T: crate::client::Client,
{
    pub fn flag(&self, flag: &str) -> flag::FlagClient<T> {
        flag::FlagClient::<T>::new(self.client.clone(), &self.path, flag)
    }
}
