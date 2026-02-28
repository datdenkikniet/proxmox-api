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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Size in GiB for block.db."]
    #[doc = ""]
    #[doc = "If a block.db is requested but the size is not given, will be automatically selected by: bluestore_block_db_size from the ceph database (osd or global section) or config (osd or global section) in that order. If this is not available, it will be sized 10% of the size of the OSD device. Fails if the available size is not enough."]
    #[doc = ""]
    pub db_dev_size: Option<DbDevSizeNum>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSD services per physical device. Only useful for fast NVMe devices"]
    #[doc = ""]
    #[doc = ".\" to utilize their performance better."]
    #[doc = ""]
    pub osds_per_device: Option<OsdsPerDeviceInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Block device name for block.wal."]
    #[doc = ""]
    pub wal_dev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Size in GiB for block.wal."]
    #[doc = ""]
    #[doc = "If a block.wal is requested but the size is not given, will be automatically selected by: bluestore_block_wal_size from the ceph database (osd or global section) or config (osd or global section) in that order. If this is not available, it will be sized 1% of the size of the OSD device. Fails if the available size is not enough."]
    #[doc = ""]
    pub wal_dev_size: Option<WalDevSizeNum>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OsdsPerDeviceInt(i128);
impl crate::types::bounded_integer::BoundedInteger for OsdsPerDeviceInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 1";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for OsdsPerDeviceInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for OsdsPerDeviceInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for OsdsPerDeviceInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DbDevSizeNum(f64);
impl crate::types::bounded_number::BoundedNumber for DbDevSizeNum {
    const MIN: Option<f64> = Some(1f64);
    const MAX: Option<f64> = None::<f64>;
    const DEFAULT: Option<f64> = None::<f64>;
    const TYPE_DESCRIPTION: &'static str = "an number greater than or equal to 1";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for DbDevSizeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for DbDevSizeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for DbDevSizeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for DbDevSizeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for DbDevSizeNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DbDevSizeNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_number(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WalDevSizeNum(f64);
impl crate::types::bounded_number::BoundedNumber for WalDevSizeNum {
    const MIN: Option<f64> = Some(0.5f64);
    const MAX: Option<f64> = None::<f64>;
    const DEFAULT: Option<f64> = None::<f64>;
    const TYPE_DESCRIPTION: &'static str = "an number greater than or equal to 0.5";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for WalDevSizeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for WalDevSizeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for WalDevSizeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for WalDevSizeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for WalDevSizeNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for WalDevSizeNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_number(deserializer)
    }
}
impl<T> OsdClient<T>
where
    T: crate::client::Client,
{
    pub fn osdid(&self, osdid: &str) -> osdid::OsdidClient<T> {
        osdid::OsdidClient::<T>::new(self.client.clone(), &self.path, osdid)
    }
}
