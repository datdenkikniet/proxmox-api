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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Command {
    #[serde(rename = "reboot")]
    Reboot,
    #[serde(rename = "shutdown")]
    Shutdown,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Mode {
    #[serde(rename = "efi")]
    Efi,
    #[serde(rename = "legacy-bios")]
    LegacyBios,
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
    pub mode: Mode,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "System is booted in secure mode, only applicable for the \"efi\" mode."]
    pub secureboot: Option<bool>,
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
    pub machine: String,
    #[doc = "OS kernel release (e.g., \"6.8.0\")"]
    pub release: String,
    #[doc = "OS kernel name (e.g., \"Linux\")"]
    pub sysname: String,
    #[doc = "OS kernel version with build info"]
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
        current_kernel: CurrentKernelGetOutputCurrentKernel,
    ) -> Self {
        Self {
            boot_info,
            current_kernel,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(rename = "boot-info")]
    #[doc = "Meta-information about the boot mode."]
    pub boot_info: BootInfoGetOutputBootInfo,
    #[serde(rename = "current-kernel")]
    #[doc = "The uptime of the system in seconds."]
    pub current_kernel: CurrentKernelGetOutputCurrentKernel,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read node status"]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
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
    pub command: Command,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Reboot or shutdown a node."]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
