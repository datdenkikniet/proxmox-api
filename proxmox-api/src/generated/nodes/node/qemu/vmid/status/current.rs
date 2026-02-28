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
    pub fn new(ha: HaGetOutputHa, status: Status, vmid: VmidInt) -> Self {
        Self {
            ha,
            status,
            vmid,
            agent: Default::default(),
            clipboard: Default::default(),
            cpus: Default::default(),
            lock: Default::default(),
            maxdisk: Default::default(),
            maxmem: Default::default(),
            name: Default::default(),
            pid: Default::default(),
            qmpstatus: Default::default(),
            running_machine: Default::default(),
            running_qemu: Default::default(),
            spice: Default::default(),
            tags: Default::default(),
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
    #[doc = "Maximum usable CPUs."]
    #[doc = ""]
    pub cpus: Option<f64>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VM name."]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "PID of running qemu process."]
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
    #[doc = "The currently running QEMU version (if running)."]
    #[doc = ""]
    pub running_qemu: Option<String>,
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
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Uptime."]
    #[doc = ""]
    pub uptime: Option<i64>,
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub vmid: VmidInt,
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
