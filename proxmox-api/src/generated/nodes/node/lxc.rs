pub mod vmid;
#[derive(Debug, Clone)]
pub struct LxcClient<T> {
    client: T,
    path: String,
}
impl<T> LxcClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/lxc"),
        }
    }
}
impl<T> LxcClient<T>
where
    T: crate::client::Client,
{
    #[doc = "LXC container index (per node)."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> LxcClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create or restore a container."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(status: Status, vmid: crate::types::VmId) -> Self {
        Self {
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
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum usable CPUs."]
    #[doc = ""]
    pub cpus: Option<f64>,
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
    #[doc = "Maximum SWAP memory in bytes."]
    #[doc = ""]
    pub maxswap: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Container name."]
    #[doc = ""]
    pub name: Option<String>,
    #[doc = "LXC Container status."]
    #[doc = ""]
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current configured tags, if any."]
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
    pub vmid: crate::types::VmId,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(ostemplate: String, vmid: crate::types::VmId) -> Self {
        Self {
            ostemplate,
            vmid,
            arch: Default::default(),
            bwlimit: Default::default(),
            cmode: Default::default(),
            console: Default::default(),
            cores: Default::default(),
            cpulimit: Default::default(),
            cpuunits: Default::default(),
            debug: Default::default(),
            description: Default::default(),
            devs: Default::default(),
            features: Default::default(),
            force: Default::default(),
            hookscript: Default::default(),
            hostname: Default::default(),
            ignore_unpack_errors: Default::default(),
            lock: Default::default(),
            memory: Default::default(),
            mps: Default::default(),
            nameserver: Default::default(),
            nets: Default::default(),
            onboot: Default::default(),
            ostype: Default::default(),
            password: Default::default(),
            pool: Default::default(),
            protection: Default::default(),
            restore: Default::default(),
            rootfs: Default::default(),
            searchdomain: Default::default(),
            ssh_public_keys: Default::default(),
            start: Default::default(),
            startup: Default::default(),
            storage: Default::default(),
            swap: Default::default(),
            tags: Default::default(),
            template: Default::default(),
            timezone: Default::default(),
            tty: Default::default(),
            unique: Default::default(),
            unprivileged: Default::default(),
            unuseds: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OS architecture type."]
    #[doc = ""]
    pub arch: Option<Arch>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Console mode. By default, the console command tries to open a connection to one of the available tty devices. By setting cmode to 'console' it tries to attach to /dev/console instead. If you set cmode to 'shell', it simply invokes a shell inside the container (no login)."]
    #[doc = ""]
    pub cmode: Option<Cmode>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Attach a console device (/dev/console) to the container."]
    #[doc = ""]
    pub console: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of cores assigned to the container. A container can use all available cores by default."]
    #[doc = ""]
    pub cores: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit of CPU usage."]
    #[doc = ""]
    #[doc = "NOTE: If the computer has 2 CPUs, it has a total of '2' CPU time. Value '0' indicates no CPU limit."]
    #[doc = ""]
    pub cpulimit: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU weight for a container, will be clamped to \\\\[1, 10000\\\\] in cgroup v2."]
    #[doc = ""]
    #[doc = "CPU weight for a container. Argument is used in the kernel fair scheduler. The larger the number is, the more CPU time this container gets. Number is relative to the weights of all the other running guests."]
    #[doc = ""]
    pub cpuunits: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Try to be more verbose. For now this only enables debug log-level on start."]
    #[doc = ""]
    pub debug: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Container. Shown in the web-interface CT's summary. This is saved as comment inside the configuration file."]
    #[doc = ""]
    pub description: Option<String>,
    #[serde(rename = "dev[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedDevs, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedDevs, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Device to pass through to the container"]
    #[doc = ""]
    pub devs: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow containers access to advanced features."]
    #[doc = ""]
    pub features: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow to overwrite existing container."]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Script that will be exectued during various steps in the containers lifetime."]
    #[doc = ""]
    pub hookscript: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set a host name for the container."]
    #[doc = ""]
    pub hostname: Option<String>,
    #[serde(rename = "ignore-unpack-errors")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Ignore errors when extracting the template."]
    #[doc = ""]
    pub ignore_unpack_errors: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Lock/unlock the container."]
    #[doc = ""]
    pub lock: Option<Lock>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Amount of RAM for the container in MB."]
    #[doc = ""]
    pub memory: Option<i64>,
    #[serde(rename = "mp[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedMps, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedMps, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Use volume as container mount point. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume."]
    #[doc = ""]
    pub mps: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets DNS server IP address for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver."]
    #[doc = ""]
    pub nameserver: Option<String>,
    #[serde(rename = "net[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedNets, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedNets, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Specifies network interfaces for the container."]
    #[doc = ""]
    pub nets: ::std::collections::HashMap<u32, String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies whether a container will be started during system bootup."]
    #[doc = ""]
    pub onboot: Option<bool>,
    #[doc = "The OS template or backup file."]
    #[doc = ""]
    pub ostemplate: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/\\\\<ostype\\\\>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup."]
    #[doc = ""]
    pub ostype: Option<Ostype>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets root password inside container."]
    #[doc = ""]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add the VM to the specified pool."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation."]
    #[doc = ""]
    pub protection: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mark this as restore task."]
    #[doc = ""]
    pub restore: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use volume as container root."]
    #[doc = ""]
    pub rootfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver."]
    #[doc = ""]
    pub searchdomain: Option<String>,
    #[serde(rename = "ssh-public-keys")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Setup public SSH keys (one key per line, OpenSSH format)."]
    #[doc = ""]
    pub ssh_public_keys: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Start the CT after its creation finished successfully."]
    #[doc = ""]
    pub start: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[doc = ""]
    pub startup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default Storage."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Amount of SWAP for the container in MB."]
    #[doc = ""]
    pub swap: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Tags of the Container. This is only meta information."]
    #[doc = ""]
    pub tags: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable Template."]
    #[doc = ""]
    pub template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Time zone to use in the container. If option isn't set, then nothing will be done. Can be set to 'host' to match the host time zone, or an arbitrary time zone option from /usr/share/zoneinfo/zone.tab"]
    #[doc = ""]
    pub timezone: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the number of tty available to the container"]
    #[doc = ""]
    pub tty: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Assign a unique random ethernet address."]
    #[doc = ""]
    pub unique: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Makes the container run as unprivileged user. (Should not be modified manually.)"]
    #[doc = ""]
    pub unprivileged: Option<bool>,
    #[serde(rename = "unused[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedUnuseds, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedUnuseds, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Reference to unused volumes. This is used internally, and should not be modified manually."]
    #[doc = ""]
    pub unuseds: ::std::collections::HashMap<u32, String>,
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub vmid: crate::types::VmId,
    #[serde(
        flatten,
        deserialize_with = "crate::types::multi::deserialize_additional_data::<'_, PostParams, _, _>"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl crate::types::multi::Test for PostParams {
    fn test_fn() -> fn(&str) -> bool {
        fn the_test(input: &str) -> bool {
            let array = [
                <NumberedDevs as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedMps as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedNets as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedUnuseds as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
            ];
            array.iter().any(|f| f(input))
        }
        the_test as _
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "OS architecture type."]
#[doc = ""]
pub enum Arch {
    #[serde(rename = "amd64")]
    Amd64,
    #[serde(rename = "arm64")]
    Arm64,
    #[serde(rename = "armhf")]
    Armhf,
    #[serde(rename = "i386")]
    I386,
    #[serde(rename = "riscv32")]
    Riscv32,
    #[serde(rename = "riscv64")]
    Riscv64,
}
impl TryFrom<&str> for Arch {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "amd64" => Ok(Self::Amd64),
            "arm64" => Ok(Self::Arm64),
            "armhf" => Ok(Self::Armhf),
            "i386" => Ok(Self::I386),
            "riscv32" => Ok(Self::Riscv32),
            "riscv64" => Ok(Self::Riscv64),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Arch {
    fn default() -> Self {
        Self::Amd64
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Console mode. By default, the console command tries to open a connection to one of the available tty devices. By setting cmode to 'console' it tries to attach to /dev/console instead. If you set cmode to 'shell', it simply invokes a shell inside the container (no login)."]
#[doc = ""]
pub enum Cmode {
    #[serde(rename = "console")]
    Console,
    #[serde(rename = "shell")]
    Shell,
    #[serde(rename = "tty")]
    Tty,
}
impl TryFrom<&str> for Cmode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "console" => Ok(Self::Console),
            "shell" => Ok(Self::Shell),
            "tty" => Ok(Self::Tty),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl Default for Cmode {
    fn default() -> Self {
        Self::Tty
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Lock/unlock the container."]
#[doc = ""]
pub enum Lock {
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "destroyed")]
    Destroyed,
    #[serde(rename = "disk")]
    Disk,
    #[serde(rename = "fstrim")]
    Fstrim,
    #[serde(rename = "migrate")]
    Migrate,
    #[serde(rename = "mounted")]
    Mounted,
    #[serde(rename = "rollback")]
    Rollback,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "snapshot-delete")]
    SnapshotDelete,
}
impl TryFrom<&str> for Lock {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "backup" => Ok(Self::Backup),
            "create" => Ok(Self::Create),
            "destroyed" => Ok(Self::Destroyed),
            "disk" => Ok(Self::Disk),
            "fstrim" => Ok(Self::Fstrim),
            "migrate" => Ok(Self::Migrate),
            "mounted" => Ok(Self::Mounted),
            "rollback" => Ok(Self::Rollback),
            "snapshot" => Ok(Self::Snapshot),
            "snapshot-delete" => Ok(Self::SnapshotDelete),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/\\<ostype\\>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup."]
#[doc = ""]
pub enum Ostype {
    #[serde(rename = "alpine")]
    Alpine,
    #[serde(rename = "archlinux")]
    Archlinux,
    #[serde(rename = "centos")]
    Centos,
    #[serde(rename = "debian")]
    Debian,
    #[serde(rename = "devuan")]
    Devuan,
    #[serde(rename = "fedora")]
    Fedora,
    #[serde(rename = "gentoo")]
    Gentoo,
    #[serde(rename = "nixos")]
    Nixos,
    #[serde(rename = "opensuse")]
    Opensuse,
    #[serde(rename = "ubuntu")]
    Ubuntu,
    #[serde(rename = "unmanaged")]
    Unmanaged,
}
impl TryFrom<&str> for Ostype {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "alpine" => Ok(Self::Alpine),
            "archlinux" => Ok(Self::Archlinux),
            "centos" => Ok(Self::Centos),
            "debian" => Ok(Self::Debian),
            "devuan" => Ok(Self::Devuan),
            "fedora" => Ok(Self::Fedora),
            "gentoo" => Ok(Self::Gentoo),
            "nixos" => Ok(Self::Nixos),
            "opensuse" => Ok(Self::Opensuse),
            "ubuntu" => Ok(Self::Ubuntu),
            "unmanaged" => Ok(Self::Unmanaged),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "LXC Container status."]
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
#[derive(Default)]
struct NumberedDevs;
impl crate::types::multi::NumberedItems for NumberedDevs {
    type Item = String;
    const PREFIX: &'static str = "dev";
}
#[derive(Default)]
struct NumberedMps;
impl crate::types::multi::NumberedItems for NumberedMps {
    type Item = String;
    const PREFIX: &'static str = "mp";
}
#[derive(Default)]
struct NumberedNets;
impl crate::types::multi::NumberedItems for NumberedNets {
    type Item = String;
    const PREFIX: &'static str = "net";
}
#[derive(Default)]
struct NumberedUnuseds;
impl crate::types::multi::NumberedItems for NumberedUnuseds {
    type Item = String;
    const PREFIX: &'static str = "unused";
}
impl<T> LxcClient<T>
where
    T: crate::client::Client,
{
    pub fn vmid(&self, vmid: crate::types::VmId) -> vmid::VmidClient<T> {
        vmid::VmidClient::<T>::new(self.client.clone(), &self.path, vmid)
    }
}
