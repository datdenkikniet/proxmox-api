pub mod storage;
#[derive(Debug, Clone)]
pub struct StorageClient<T> {
    client: T,
    path: String,
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/storage"),
        }
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get status for all datastores."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl FormatsGetOutputItemsFormats {
    pub fn new(default: Default, supported: Vec<Supported>) -> Self {
        Self {
            default,
            supported,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct FormatsGetOutputItemsFormats {
    #[doc = "The default format of the storage."]
    #[doc = ""]
    pub default: Default,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The list of supported formats"]
    #[doc = ""]
    pub supported: Vec<Supported>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(content: String, storage: String, ty: String) -> Self {
        Self {
            content,
            storage,
            ty,
            active: ::std::default::Default::default(),
            avail: ::std::default::Default::default(),
            enabled: ::std::default::Default::default(),
            formats: ::std::default::Default::default(),
            select_existing: ::std::default::Default::default(),
            shared: ::std::default::Default::default(),
            total: ::std::default::Default::default(),
            used: ::std::default::Default::default(),
            used_fraction: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set when storage is accessible."]
    #[doc = ""]
    pub active: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Available storage space in bytes."]
    #[doc = ""]
    pub avail: Option<i64>,
    #[doc = "Allowed storage content types."]
    #[doc = ""]
    pub content: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set when storage is enabled (not disabled)."]
    #[doc = ""]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Lists the supported and default format. Use 'formats' instead. Only included if 'format' parameter is set."]
    #[doc = ""]
    pub formats: Option<FormatsGetOutputItemsFormats>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Instead of creating new volumes, one must select one that is already existing. Only included if 'format' parameter is set."]
    #[doc = ""]
    pub select_existing: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Shared flag from storage configuration."]
    #[doc = ""]
    pub shared: Option<bool>,
    #[doc = "The storage identifier."]
    #[doc = ""]
    pub storage: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Total storage space in bytes."]
    #[doc = ""]
    pub total: Option<i64>,
    #[serde(rename = "type")]
    #[doc = "Storage type."]
    #[doc = ""]
    pub ty: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used storage space in bytes."]
    #[doc = ""]
    pub used: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used fraction (used/total)."]
    #[doc = ""]
    pub used_fraction: Option<f64>,
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
    #[doc = "Only list stores which support this content type."]
    #[doc = ""]
    pub content: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list stores which are enabled (not disabled in config)."]
    #[doc = ""]
    pub enabled: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Include information about formats"]
    #[doc = ""]
    pub format: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list status for  specified storage"]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If target is different to 'node', we only lists shared storages which content is accessible on this 'node' and the specified 'target' node."]
    #[doc = ""]
    pub target: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The default format of the storage."]
#[doc = ""]
pub enum Default {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "subvol")]
    Subvol,
    #[serde(rename = "vmdk")]
    Vmdk,
}
impl TryFrom<&str> for Default {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "qcow2" => Ok(Self::Qcow2),
            "raw" => Ok(Self::Raw),
            "subvol" => Ok(Self::Subvol),
            "vmdk" => Ok(Self::Vmdk),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Supported {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "subvol")]
    Subvol,
    #[serde(rename = "vmdk")]
    Vmdk,
}
impl TryFrom<&str> for Supported {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "qcow2" => Ok(Self::Qcow2),
            "raw" => Ok(Self::Raw),
            "subvol" => Ok(Self::Subvol),
            "vmdk" => Ok(Self::Vmdk),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn storage(&self, storage: &str) -> storage::StorageClient<T> {
        storage::StorageClient::<T>::new(self.client.clone(), &self.path, storage)
    }
}
