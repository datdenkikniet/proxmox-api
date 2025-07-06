#[derive(Debug, Clone)]
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
impl<T> CurrentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get virtual machine status."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutput {
    pub fn new(ha: HaGetOutputHa, status: Status, vmid: crate::types::VmId) -> Self {
        Self {
            ha,
            status,
            vmid,
            agent: Default::default(),
            clipboard: Default::default(),
            cpu: Default::default(),
            cpus: Default::default(),
            diskread: Default::default(),
            diskwrite: Default::default(),
            lock: Default::default(),
            maxdisk: Default::default(),
            maxmem: Default::default(),
            mem: Default::default(),
            name: Default::default(),
            netin: Default::default(),
            netout: Default::default(),
            pid: Default::default(),
            qmpstatus: Default::default(),
            running_machine: Default::default(),
            running_qemu: Default::default(),
            serial: Default::default(),
            spice: Default::default(),
            tags: Default::default(),
            template: Default::default(),
            uptime: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "QEMU Guest Agent is enabled in config."]
    #[doc = ""]
    pub agent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable a specific clipboard. If not set, depending on the display type the SPICE one will be added."]
    #[doc = ""]
    pub clipboard: Option<Clipboard>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Current CPU usage."]
    #[doc = ""]
    pub cpu: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum usable CPUs."]
    #[doc = ""]
    pub cpus: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The amount of bytes the guest read from it's block devices since the guest was started. (Note: This info is not available for all storage types.)"]
    #[doc = ""]
    pub diskread: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The amount of bytes the guest wrote from it's block devices since the guest was started. (Note: This info is not available for all storage types.)"]
    #[doc = ""]
    pub diskwrite: Option<i64>,
    #[doc = "HA manager service status."]
    #[doc = ""]
    pub ha: HaGetOutputHa,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current config lock, if any."]
    #[doc = ""]
    pub lock: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Root disk size in bytes."]
    #[doc = ""]
    pub maxdisk: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum memory in bytes."]
    #[doc = ""]
    pub maxmem: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Currently used memory in bytes."]
    #[doc = ""]
    pub mem: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VM (host)name."]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The amount of traffic in bytes that was sent to the guest over the network since it was started."]
    #[doc = ""]
    pub netin: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The amount of traffic in bytes that was sent from the guest over the network since it was started."]
    #[doc = ""]
    pub netout: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "PID of the QEMU process, if the VM is running."]
    #[doc = ""]
    pub pid: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VM run state from the 'query-status' QMP monitor command."]
    #[doc = ""]
    pub qmpstatus: Option<String>,
    #[serde(rename = "running-machine")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The currently running machine type (if running)."]
    #[doc = ""]
    pub running_machine: Option<String>,
    #[serde(rename = "running-qemu")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The QEMU version the VM is currently using (if running)."]
    #[doc = ""]
    pub running_qemu: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Guest has serial device configured."]
    #[doc = ""]
    pub serial: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "QEMU VGA configuration supports spice."]
    #[doc = ""]
    pub spice: Option<bool>,
    #[doc = "QEMU process status."]
    #[doc = ""]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current configured tags, if any"]
    #[doc = ""]
    pub tags: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determines if the guest is a template."]
    #[doc = ""]
    pub template: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Uptime in seconds."]
    #[doc = ""]
    pub uptime: Option<i64>,
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub vmid: crate::types::VmId,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Enable a specific clipboard. If not set, depending on the display type the SPICE one will be added."]
#[doc = ""]
pub enum Clipboard {
    #[serde(rename = "vnc")]
    Vnc,
}
impl TryFrom<&str> for Clipboard {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "vnc" => Ok(Self::Vnc),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "QEMU process status."]
#[doc = ""]
pub enum Status {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
}
impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "running" => Ok(Self::Running),
            "stopped" => Ok(Self::Stopped),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
