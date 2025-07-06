pub mod osdid;
#[derive(Debug, Clone)]
pub struct OsdClient<T> {
    client: T,
    path: String,
}
impl<T> OsdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/osd"),
        }
    }
}
impl<T> OsdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get Ceph osd list/tree."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> OsdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create OSD"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(dev: String) -> Self {
        Self {
            dev,
            crush_device_class: Default::default(),
            db_dev: Default::default(),
            db_dev_size: Default::default(),
            encrypted: Default::default(),
            osds_per_device: Default::default(),
            wal_dev: Default::default(),
            wal_dev_size: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(rename = "crush-device-class")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set the device class of the OSD in crush."]
    #[doc = ""]
    pub crush_device_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Block device name for block.db."]
    #[doc = ""]
    pub db_dev: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Size in GiB for block.db."]
    #[doc = ""]
    #[doc = "If a block.db is requested but the size is not given, will be automatically selected by: bluestore_block_db_size from the ceph database (osd or global section) or config (osd or global section) in that order. If this is not available, it will be sized 10% of the size of the OSD device. Fails if the available size is not enough."]
    #[doc = ""]
    pub db_dev_size: Option<f64>,
    #[doc = "Block device name."]
    #[doc = ""]
    pub dev: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enables encryption of the OSD."]
    #[doc = ""]
    pub encrypted: Option<bool>,
    #[serde(rename = "osds-per-device")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSD services per physical device. Only useful for fast NVMe devices"]
    #[doc = ""]
    #[doc = ".\" to utilize their performance better."]
    #[doc = ""]
    pub osds_per_device: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Block device name for block.wal."]
    #[doc = ""]
    pub wal_dev: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Size in GiB for block.wal."]
    #[doc = ""]
    #[doc = "If a block.wal is requested but the size is not given, will be automatically selected by: bluestore_block_wal_size from the ceph database (osd or global section) or config (osd or global section) in that order. If this is not available, it will be sized 1% of the size of the OSD device. Fails if the available size is not enough."]
    #[doc = ""]
    pub wal_dev_size: Option<f64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> OsdClient<T>
where
    T: crate::client::Client,
{
    pub fn osdid(&self, osdid: &str) -> osdid::OsdidClient<T> {
        osdid::OsdidClient::<T>::new(self.client.clone(), &self.path, osdid)
    }
}
