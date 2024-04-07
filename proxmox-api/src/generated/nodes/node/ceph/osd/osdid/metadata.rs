pub struct MetadataClient<T> {
    client: T,
    path: String,
}
impl<T> MetadataClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/metadata"),
        }
    }
}
impl<T> MetadataClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get OSD details"]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl DevicesGetOutputDevicesItems {
    pub fn new(
        dev_node: String,
        device: Device,
        devices: String,
        size: u64,
        support_discard: bool,
        ty: String,
    ) -> Self {
        Self {
            dev_node,
            device,
            devices,
            size,
            support_discard,
            ty,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct DevicesGetOutputDevicesItems {
    #[doc = "Device node"]
    pub dev_node: String,
    #[doc = "Kind of OSD device"]
    pub device: Device,
    #[doc = "Physical disks used"]
    pub devices: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Size in bytes"]
    pub size: u64,
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Discard support of the physical device"]
    pub support_discard: bool,
    #[serde(rename = "type")]
    #[doc = "Type of device. For example, hdd or ssd"]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(devices: Vec<DevicesGetOutputDevicesItems>, osd: OsdGetOutputOsd) -> Self {
        Self {
            devices,
            osd,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Array containing data about devices"]
    pub devices: Vec<DevicesGetOutputDevicesItems>,
    #[doc = "General information about the OSD"]
    pub osd: OsdGetOutputOsd,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl OsdGetOutputOsd {
    pub fn new(
        back_addr: String,
        front_addr: String,
        hb_back_addr: String,
        hb_front_addr: String,
        hostname: String,
        id: u64,
        mem_usage: u64,
        osd_data: String,
        osd_objectstore: String,
        pid: u64,
        version: String,
    ) -> Self {
        Self {
            back_addr,
            front_addr,
            hb_back_addr,
            hb_front_addr,
            hostname,
            id,
            mem_usage,
            osd_data,
            osd_objectstore,
            pid,
            version,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct OsdGetOutputOsd {
    #[doc = "Address and port used to talk to other OSDs."]
    pub back_addr: String,
    #[doc = "Address and port used to talk to clients and monitors."]
    pub front_addr: String,
    #[doc = "Heartbeat address and port for other OSDs."]
    pub hb_back_addr: String,
    #[doc = "Heartbeat address and port for clients and monitors."]
    pub hb_front_addr: String,
    #[doc = "Name of the host containing the OSD."]
    pub hostname: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "ID of the OSD."]
    pub id: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Memory usage of the OSD service."]
    pub mem_usage: u64,
    #[doc = "Path to the OSD's data directory."]
    pub osd_data: String,
    #[doc = "The type of object store used."]
    pub osd_objectstore: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "OSD process ID."]
    pub pid: u64,
    #[doc = "Ceph version of the OSD service."]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Device {
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "db")]
    Db,
    #[serde(rename = "wal")]
    Wal,
}
