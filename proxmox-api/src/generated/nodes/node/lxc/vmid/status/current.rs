pub struct CurrentClient<T> {
    client: T,
    path: String,
}
impl<T> CurrentClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/current"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Status {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct HaGetOutputHa {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(ha: HaGetOutputHa, status: Status, vmid: crate::VmId) -> Self {
        Self {
            ha,
            status,
            vmid,
            cpus: Default::default(),
            lock: Default::default(),
            maxdisk: Default::default(),
            maxmem: Default::default(),
            maxswap: Default::default(),
            name: Default::default(),
            tags: Default::default(),
            uptime: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(
        serialize_with = "crate::serialize_number_optional",
        deserialize_with = "crate::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum usable CPUs."]
    pub cpus: Option<f64>,
    #[doc = "HA manager service status."]
    pub ha: HaGetOutputHa,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current config lock, if any."]
    pub lock: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Root disk size in bytes."]
    pub maxdisk: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum memory in bytes."]
    pub maxmem: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum SWAP memory in bytes."]
    pub maxswap: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Container name."]
    pub name: Option<String>,
    #[doc = "LXC Container status."]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current configured tags, if any."]
    pub tags: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Uptime."]
    pub uptime: Option<u64>,
    #[doc = "The (unique) ID of the VM."]
    pub vmid: crate::VmId,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> CurrentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get virtual machine status."]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
