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
impl<'a, T> crate::ProxmoxClient for &'a StatusClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read node status"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), GetOutput, T::Error> for &StatusClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<GetOutput, T::Error> {
        self.get()
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Reboot or shutdown a node."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, (), T::Error> for &StatusClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<(), T::Error> {
        self.post(params)
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
    #[doc = ""]
    pub boot_info: BootInfoGetOutputBootInfo,
    #[serde(rename = "current-kernel")]
    #[doc = "The uptime of the system in seconds."]
    #[doc = ""]
    pub current_kernel: CurrentKernelGetOutputCurrentKernel,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
