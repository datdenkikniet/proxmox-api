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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
    pub description: String,
    #[doc = "Flag name."]
    pub name: Name,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Flag value."]
    pub value: bool,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> FlagsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "get the status of all ceph flags"]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backfilling of PGs is suspended."]
    pub nobackfill: Option<bool>,
    #[serde(rename = "nodeep-scrub")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deep Scrubbing is disabled."]
    pub nodeep_scrub: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSD failure reports are being ignored, such that the monitors will not mark OSDs down."]
    pub nodown: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSDs that were previously marked out will not be marked back in when they start."]
    pub noin: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSDs will not automatically be marked out after the configured interval."]
    pub noout: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Rebalancing of PGs is suspended."]
    pub norebalance: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Recovery of PGs is suspended."]
    pub norecover: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Scrubbing is disabled."]
    pub noscrub: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Cache tiering activity is suspended."]
    pub notieragent: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSDs are not allowed to start."]
    pub noup: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pauses read and writes."]
    pub pause: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> FlagsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set/Unset multiple ceph flags at once."]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
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
