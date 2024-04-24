pub mod exec;
pub mod exec_status;
pub mod file_read;
pub mod file_write;
pub mod fsfreeze_freeze;
pub mod fsfreeze_status;
pub mod fsfreeze_thaw;
pub mod fstrim;
pub mod get_fsinfo;
pub mod get_host_name;
pub mod get_memory_block_info;
pub mod get_memory_blocks;
pub mod get_osinfo;
pub mod get_time;
pub mod get_timezone;
pub mod get_users;
pub mod get_vcpus;
pub mod info;
pub mod network_get_interfaces;
pub mod ping;
pub mod set_user_password;
pub mod shutdown;
pub mod suspend_disk;
pub mod suspend_hybrid;
pub mod suspend_ram;
#[derive(Debug, Clone)]
pub struct AgentClient<T> {
    client: T,
    path: String,
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/agent"),
        }
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "QEMU Guest Agent command index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Execute QEMU Guest Agent commands."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostOutput {
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
    #[doc = "The QGA command."]
    #[doc = ""]
    pub command: Command,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The QGA command."]
#[doc = ""]
pub enum Command {
    #[serde(rename = "fsfreeze-freeze")]
    FsfreezeFreeze,
    #[serde(rename = "fsfreeze-status")]
    FsfreezeStatus,
    #[serde(rename = "fsfreeze-thaw")]
    FsfreezeThaw,
    #[serde(rename = "fstrim")]
    Fstrim,
    #[serde(rename = "get-fsinfo")]
    GetFsinfo,
    #[serde(rename = "get-host-name")]
    GetHostName,
    #[serde(rename = "get-memory-block-info")]
    GetMemoryBlockInfo,
    #[serde(rename = "get-memory-blocks")]
    GetMemoryBlocks,
    #[serde(rename = "get-osinfo")]
    GetOsinfo,
    #[serde(rename = "get-time")]
    GetTime,
    #[serde(rename = "get-timezone")]
    GetTimezone,
    #[serde(rename = "get-users")]
    GetUsers,
    #[serde(rename = "get-vcpus")]
    GetVcpus,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "network-get-interfaces")]
    NetworkGetInterfaces,
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "suspend-disk")]
    SuspendDisk,
    #[serde(rename = "suspend-hybrid")]
    SuspendHybrid,
    #[serde(rename = "suspend-ram")]
    SuspendRam,
}
impl TryFrom<&str> for Command {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "fsfreeze-freeze" => Ok(Self::FsfreezeFreeze),
            "fsfreeze-status" => Ok(Self::FsfreezeStatus),
            "fsfreeze-thaw" => Ok(Self::FsfreezeThaw),
            "fstrim" => Ok(Self::Fstrim),
            "get-fsinfo" => Ok(Self::GetFsinfo),
            "get-host-name" => Ok(Self::GetHostName),
            "get-memory-block-info" => Ok(Self::GetMemoryBlockInfo),
            "get-memory-blocks" => Ok(Self::GetMemoryBlocks),
            "get-osinfo" => Ok(Self::GetOsinfo),
            "get-time" => Ok(Self::GetTime),
            "get-timezone" => Ok(Self::GetTimezone),
            "get-users" => Ok(Self::GetUsers),
            "get-vcpus" => Ok(Self::GetVcpus),
            "info" => Ok(Self::Info),
            "network-get-interfaces" => Ok(Self::NetworkGetInterfaces),
            "ping" => Ok(Self::Ping),
            "shutdown" => Ok(Self::Shutdown),
            "suspend-disk" => Ok(Self::SuspendDisk),
            "suspend-hybrid" => Ok(Self::SuspendHybrid),
            "suspend-ram" => Ok(Self::SuspendRam),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn fsfreeze_freeze(&self) -> fsfreeze_freeze::FsfreezeFreezeClient<T> {
        fsfreeze_freeze::FsfreezeFreezeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn fsfreeze_status(&self) -> fsfreeze_status::FsfreezeStatusClient<T> {
        fsfreeze_status::FsfreezeStatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn fsfreeze_thaw(&self) -> fsfreeze_thaw::FsfreezeThawClient<T> {
        fsfreeze_thaw::FsfreezeThawClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn fstrim(&self) -> fstrim::FstrimClient<T> {
        fstrim::FstrimClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_fsinfo(&self) -> get_fsinfo::GetFsinfoClient<T> {
        get_fsinfo::GetFsinfoClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_host_name(&self) -> get_host_name::GetHostNameClient<T> {
        get_host_name::GetHostNameClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_memory_block_info(&self) -> get_memory_block_info::GetMemoryBlockInfoClient<T> {
        get_memory_block_info::GetMemoryBlockInfoClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_memory_blocks(&self) -> get_memory_blocks::GetMemoryBlocksClient<T> {
        get_memory_blocks::GetMemoryBlocksClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_osinfo(&self) -> get_osinfo::GetOsinfoClient<T> {
        get_osinfo::GetOsinfoClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_time(&self) -> get_time::GetTimeClient<T> {
        get_time::GetTimeClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_timezone(&self) -> get_timezone::GetTimezoneClient<T> {
        get_timezone::GetTimezoneClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_users(&self) -> get_users::GetUsersClient<T> {
        get_users::GetUsersClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn get_vcpus(&self) -> get_vcpus::GetVcpusClient<T> {
        get_vcpus::GetVcpusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn info(&self) -> info::InfoClient<T> {
        info::InfoClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn network_get_interfaces(&self) -> network_get_interfaces::NetworkGetInterfacesClient<T> {
        network_get_interfaces::NetworkGetInterfacesClient::<T>::new(
            self.client.clone(),
            &self.path,
        )
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn ping(&self) -> ping::PingClient<T> {
        ping::PingClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn shutdown(&self) -> shutdown::ShutdownClient<T> {
        shutdown::ShutdownClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn suspend_disk(&self) -> suspend_disk::SuspendDiskClient<T> {
        suspend_disk::SuspendDiskClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn suspend_hybrid(&self) -> suspend_hybrid::SuspendHybridClient<T> {
        suspend_hybrid::SuspendHybridClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn suspend_ram(&self) -> suspend_ram::SuspendRamClient<T> {
        suspend_ram::SuspendRamClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn set_user_password(&self) -> set_user_password::SetUserPasswordClient<T> {
        set_user_password::SetUserPasswordClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn exec(&self) -> exec::ExecClient<T> {
        exec::ExecClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn exec_status(&self) -> exec_status::ExecStatusClient<T> {
        exec_status::ExecStatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn file_read(&self) -> file_read::FileReadClient<T> {
        file_read::FileReadClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> AgentClient<T>
where
    T: crate::client::Client,
{
    pub fn file_write(&self) -> file_write::FileWriteClient<T> {
        file_write::FileWriteClient::<T>::new(self.client.clone(), &self.path)
    }
}
