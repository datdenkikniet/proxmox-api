#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/status"),
        }
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read node status"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Reboot or shutdown a node."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl BootInfoGetOutputBootInfo {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            secureboot: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct BootInfoGetOutputBootInfo {
    #[doc = "Through which firmware the system got booted."]
    #[doc = ""]
    pub mode: Mode,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "System is booted in secure mode, only applicable for the \"efi\" mode."]
    #[doc = ""]
    pub secureboot: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl CpuinfoGetOutputCpuinfo {
    pub fn new(cores: i64, cpus: i64, model: String, sockets: i64) -> Self {
        Self {
            cores,
            cpus,
            model,
            sockets,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct CpuinfoGetOutputCpuinfo {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The number of physical cores of the CPU."]
    #[doc = ""]
    pub cores: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The number of logical threads of the CPU."]
    #[doc = ""]
    pub cpus: i64,
    #[doc = "The CPU model"]
    #[doc = ""]
    pub model: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The number of logical threads of the CPU."]
    #[doc = ""]
    pub sockets: i64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl CurrentKernelGetOutputCurrentKernel {
    pub fn new(machine: String, release: String, sysname: String, version: String) -> Self {
        Self {
            machine,
            release,
            sysname,
            version,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct CurrentKernelGetOutputCurrentKernel {
    #[doc = "Hardware (architecture) type"]
    #[doc = ""]
    pub machine: String,
    #[doc = "OS kernel release (e.g., \"6.8.0\")"]
    #[doc = ""]
    pub release: String,
    #[doc = "OS kernel name (e.g., \"Linux\")"]
    #[doc = ""]
    pub sysname: String,
    #[doc = "OS kernel version with build info"]
    #[doc = ""]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(
        boot_info: BootInfoGetOutputBootInfo,
        cpu: f64,
        cpuinfo: CpuinfoGetOutputCpuinfo,
        current_kernel: CurrentKernelGetOutputCurrentKernel,
        loadavg: Vec<String>,
        memory: MemoryGetOutputMemory,
        pveversion: String,
        rootfs: RootfsGetOutputRootfs,
    ) -> Self {
        Self {
            boot_info,
            cpu,
            cpuinfo,
            current_kernel,
            loadavg,
            memory,
            pveversion,
            rootfs,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(rename = "boot-info")]
    #[doc = "Meta-information about the boot mode."]
    #[doc = ""]
    pub boot_info: BootInfoGetOutputBootInfo,
    #[serde(
        serialize_with = "crate::types::serialize_number",
        deserialize_with = "crate::types::deserialize_number"
    )]
    #[doc = "The current cpu usage."]
    #[doc = ""]
    pub cpu: f64,
    pub cpuinfo: CpuinfoGetOutputCpuinfo,
    #[serde(rename = "current-kernel")]
    #[doc = "Meta-information about the currently booted kernel of this node."]
    #[doc = ""]
    pub current_kernel: CurrentKernelGetOutputCurrentKernel,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "An array of load avg for 1, 5 and 15 minutes respectively."]
    #[doc = ""]
    pub loadavg: Vec<String>,
    pub memory: MemoryGetOutputMemory,
    #[doc = "The PVE version string."]
    #[doc = ""]
    pub pveversion: String,
    pub rootfs: RootfsGetOutputRootfs,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl MemoryGetOutputMemory {
    pub fn new(free: i64, total: i64, used: i64) -> Self {
        Self {
            free,
            total,
            used,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct MemoryGetOutputMemory {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The free memory in bytes."]
    #[doc = ""]
    pub free: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The total memory in bytes."]
    #[doc = ""]
    pub total: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The used memory in bytes."]
    #[doc = ""]
    pub used: i64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(command: Command) -> Self {
        Self {
            command,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Specify the command."]
    #[doc = ""]
    pub command: Command,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl RootfsGetOutputRootfs {
    pub fn new(avail: i64, free: i64, total: i64, used: i64) -> Self {
        Self {
            avail,
            free,
            total,
            used,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct RootfsGetOutputRootfs {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The available bytes in the root filesystem."]
    #[doc = ""]
    pub avail: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The free bytes on the root filesystem."]
    #[doc = ""]
    pub free: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The total size of the root filesystem in bytes."]
    #[doc = ""]
    pub total: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "The used bytes in the root filesystem."]
    #[doc = ""]
    pub used: i64,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Specify the command."]
#[doc = ""]
pub enum Command {
    #[serde(rename = "reboot")]
    Reboot,
    #[serde(rename = "shutdown")]
    Shutdown,
}
impl TryFrom<&str> for Command {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "reboot" => Ok(Self::Reboot),
            "shutdown" => Ok(Self::Shutdown),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Through which firmware the system got booted."]
#[doc = ""]
pub enum Mode {
    #[serde(rename = "efi")]
    Efi,
    #[serde(rename = "legacy-bios")]
    LegacyBios,
}
impl TryFrom<&str> for Mode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "efi" => Ok(Self::Efi),
            "legacy-bios" => Ok(Self::LegacyBios),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
