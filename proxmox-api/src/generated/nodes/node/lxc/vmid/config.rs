#[derive(Debug, Clone)]
pub struct ConfigClient<T> {
    client: T,
    path: String,
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/config"),
        }
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get container configuration."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ConfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set container options."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl GetOutput {
    pub fn new(digest: String) -> Self {
        Self {
            digest,
            arch: Default::default(),
            cmode: Default::default(),
            console: Default::default(),
            cores: Default::default(),
            cpulimit: Default::default(),
            cpuunits: Default::default(),
            debug: Default::default(),
            description: Default::default(),
            devs: Default::default(),
            features: Default::default(),
            hookscript: Default::default(),
            hostname: Default::default(),
            lock: Default::default(),
            lxc: Default::default(),
            memory: Default::default(),
            mps: Default::default(),
            nameserver: Default::default(),
            nets: Default::default(),
            onboot: Default::default(),
            ostype: Default::default(),
            protection: Default::default(),
            rootfs: Default::default(),
            searchdomain: Default::default(),
            startup: Default::default(),
            swap: Default::default(),
            tags: Default::default(),
            template: Default::default(),
            timezone: Default::default(),
            tty: Default::default(),
            unprivileged: Default::default(),
            unuseds: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OS architecture type."]
    #[doc = ""]
    pub arch: Option<Arch>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of cores assigned to the container. A container can use all available cores by default."]
    #[doc = ""]
    pub cores: Option<CoresInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit of CPU usage."]
    #[doc = ""]
    #[doc = "NOTE: If the computer has 2 CPUs, it has a total of '2' CPU time. Value '0' indicates no CPU limit."]
    #[doc = ""]
    pub cpulimit: Option<CpulimitNum>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU weight for a container, will be clamped to \\\\[1, 10000\\\\] in cgroup v2."]
    #[doc = ""]
    #[doc = "CPU weight for a container. Argument is used in the kernel fair scheduler. The larger the number is, the more CPU time this container gets. Number is relative to the weights of all the other running guests."]
    #[doc = ""]
    pub cpuunits: Option<CpuunitsInt>,
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
    pub description: Option<DescriptionStr>,
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
    #[doc = "SHA1 digest of configuration file. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow containers access to advanced features."]
    #[doc = ""]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Script that will be exectued during various steps in the containers lifetime."]
    #[doc = ""]
    pub hookscript: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set a host name for the container."]
    #[doc = ""]
    pub hostname: Option<HostnameStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Lock/unlock the container."]
    #[doc = ""]
    pub lock: Option<Lock>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Array of lxc low-level configurations (\\\\[\\\\[key1, value1\\\\], \\\\[key2, value2\\\\] ...\\\\])."]
    #[doc = ""]
    pub lxc: Vec<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Amount of RAM for the container in MB."]
    #[doc = ""]
    pub memory: Option<MemoryInt>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/\\\\<ostype\\\\>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup."]
    #[doc = ""]
    pub ostype: Option<Ostype>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation."]
    #[doc = ""]
    pub protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use volume as container root."]
    #[doc = ""]
    pub rootfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver."]
    #[doc = ""]
    pub searchdomain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[doc = ""]
    pub startup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Amount of SWAP for the container in MB."]
    #[doc = ""]
    pub swap: Option<SwapInt>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the number of tty available to the container"]
    #[doc = ""]
    pub tty: Option<TtyInt>,
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
    #[serde(
        flatten,
        deserialize_with = "crate::types::multi::deserialize_additional_data::<'_, GetOutput, _, _>"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl crate::types::multi::Test for GetOutput {
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Get current values (instead of pending values)."]
    #[doc = ""]
    pub current: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Fetch config values from given snapshot."]
    #[doc = ""]
    pub snapshot: Option<SnapshotStr>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OS architecture type."]
    #[doc = ""]
    pub arch: Option<Arch>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of cores assigned to the container. A container can use all available cores by default."]
    #[doc = ""]
    pub cores: Option<CoresInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit of CPU usage."]
    #[doc = ""]
    #[doc = "NOTE: If the computer has 2 CPUs, it has a total of '2' CPU time. Value '0' indicates no CPU limit."]
    #[doc = ""]
    pub cpulimit: Option<CpulimitNum>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU weight for a container, will be clamped to \\\\[1, 10000\\\\] in cgroup v2."]
    #[doc = ""]
    #[doc = "CPU weight for a container. Argument is used in the kernel fair scheduler. The larger the number is, the more CPU time this container gets. Number is relative to the weights of all the other running guests."]
    #[doc = ""]
    pub cpuunits: Option<CpuunitsInt>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Try to be more verbose. For now this only enables debug log-level on start."]
    #[doc = ""]
    pub debug: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Container. Shown in the web-interface CT's summary. This is saved as comment inside the configuration file."]
    #[doc = ""]
    pub description: Option<DescriptionStr>,
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
    #[doc = "Prevent changes if current configuration file has different SHA1 digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow containers access to advanced features."]
    #[doc = ""]
    pub features: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Script that will be exectued during various steps in the containers lifetime."]
    #[doc = ""]
    pub hookscript: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set a host name for the container."]
    #[doc = ""]
    pub hostname: Option<HostnameStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Lock/unlock the container."]
    #[doc = ""]
    pub lock: Option<Lock>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Amount of RAM for the container in MB."]
    #[doc = ""]
    pub memory: Option<MemoryInt>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OS type. This is used to setup configuration inside the container, and corresponds to lxc setup scripts in /usr/share/lxc/config/\\\\<ostype\\\\>.common.conf. Value 'unmanaged' can be used to skip and OS specific setup."]
    #[doc = ""]
    pub ostype: Option<Ostype>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets the protection flag of the container. This will prevent the CT or CT's disk remove/update operation."]
    #[doc = ""]
    pub protection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Revert a pending change."]
    #[doc = ""]
    pub revert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use volume as container root."]
    #[doc = ""]
    pub rootfs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets DNS search domains for a container. Create will automatically use the setting from the host if you neither set searchdomain nor nameserver."]
    #[doc = ""]
    pub searchdomain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[doc = ""]
    pub startup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Amount of SWAP for the container in MB."]
    #[doc = ""]
    pub swap: Option<SwapInt>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the number of tty available to the container"]
    #[doc = ""]
    pub tty: Option<TtyInt>,
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
    #[serde(
        flatten,
        deserialize_with = "crate::types::multi::deserialize_additional_data::<'_, PutParams, _, _>"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl crate::types::multi::Test for PutParams {
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "OS architecture type."]
#[doc = ""]
pub enum Arch {
    #[serde(rename = "amd64")]
    #[default]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Console mode. By default, the console command tries to open a connection to one of the available tty devices. By setting cmode to 'console' it tries to attach to /dev/console instead. If you set cmode to 'shell', it simply invokes a shell inside the container (no login)."]
#[doc = ""]
pub enum Cmode {
    #[serde(rename = "console")]
    Console,
    #[serde(rename = "shell")]
    Shell,
    #[serde(rename = "tty")]
    #[default]
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CoresInt(i128);
impl crate::types::bounded_integer::BoundedInteger for CoresInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(8192i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 8192";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for CoresInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for CoresInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CoresInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CpuunitsInt(i128);
impl crate::types::bounded_integer::BoundedInteger for CpuunitsInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = Some(500000i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 0 and 500000";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for CpuunitsInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for CpuunitsInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CpuunitsInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MemoryInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MemoryInt {
    const MIN: Option<i128> = Some(16i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(512i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 16";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MemoryInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MemoryInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MemoryInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SwapInt(i128);
impl crate::types::bounded_integer::BoundedInteger for SwapInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(512i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for SwapInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for SwapInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SwapInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TtyInt(i128);
impl crate::types::bounded_integer::BoundedInteger for TtyInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = Some(6i128);
    const DEFAULT: Option<i128> = Some(2i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 0 and 6";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for TtyInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for TtyInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for TtyInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CpulimitNum(f64);
impl crate::types::bounded_number::BoundedNumber for CpulimitNum {
    const MIN: Option<f64> = Some(0f64);
    const MAX: Option<f64> = Some(8192f64);
    const DEFAULT: Option<f64> = Some(0f64);
    const TYPE_DESCRIPTION: &'static str = "an number between 0 and 8192";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for CpulimitNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for CpulimitNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for CpulimitNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for CpulimitNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for CpulimitNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CpulimitNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_number(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DescriptionStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DescriptionStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(8192usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 8192";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DescriptionStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DescriptionStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DescriptionStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DigestStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DigestStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(40usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 40";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DigestStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DigestStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DigestStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HostnameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for HostnameStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(255usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 255";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for HostnameStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for HostnameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for HostnameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SnapshotStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for SnapshotStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(40usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 40";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for SnapshotStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for SnapshotStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SnapshotStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
