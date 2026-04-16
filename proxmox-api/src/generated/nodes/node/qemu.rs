pub mod vmid;
#[derive(Debug, Clone)]
pub struct QemuClient<T> {
    client: T,
    path: String,
}
impl<T> QemuClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/qemu"),
        }
    }
}
impl<T> QemuClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Virtual machine index (per node)."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> QemuClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create or restore a virtual machine."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(status: Status, vmid: VmidInt) -> Self {
        Self {
            status,
            vmid,
            cpu: ::std::default::Default::default(),
            cpus: ::std::default::Default::default(),
            diskread: ::std::default::Default::default(),
            diskwrite: ::std::default::Default::default(),
            lock: ::std::default::Default::default(),
            maxdisk: ::std::default::Default::default(),
            maxmem: ::std::default::Default::default(),
            mem: ::std::default::Default::default(),
            memhost: ::std::default::Default::default(),
            name: ::std::default::Default::default(),
            netin: ::std::default::Default::default(),
            netout: ::std::default::Default::default(),
            pid: ::std::default::Default::default(),
            pressurecpufull: ::std::default::Default::default(),
            pressurecpusome: ::std::default::Default::default(),
            pressureiofull: ::std::default::Default::default(),
            pressureiosome: ::std::default::Default::default(),
            pressurememoryfull: ::std::default::Default::default(),
            pressurememorysome: ::std::default::Default::default(),
            qmpstatus: ::std::default::Default::default(),
            running_machine: ::std::default::Default::default(),
            running_qemu: ::std::default::Default::default(),
            serial: ::std::default::Default::default(),
            tags: ::std::default::Default::default(),
            template: ::std::default::Default::default(),
            uptime: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
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
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Current memory usage on the host."]
    #[doc = ""]
    pub memhost: Option<i64>,
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
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU Full pressure stall average over the last 10 seconds."]
    #[doc = ""]
    pub pressurecpufull: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU Some pressure stall average over the last 10 seconds."]
    #[doc = ""]
    pub pressurecpusome: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IO Full pressure stall average over the last 10 seconds."]
    #[doc = ""]
    pub pressureiofull: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IO Some pressure stall average over the last 10 seconds."]
    #[doc = ""]
    pub pressureiosome: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Memory Full pressure stall average over the last 10 seconds."]
    #[doc = ""]
    pub pressurememoryfull: Option<f64>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Memory Some pressure stall average over the last 10 seconds."]
    #[doc = ""]
    pub pressurememorysome: Option<f64>,
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
    pub vmid: VmidInt,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determine the full status of active VMs."]
    #[doc = ""]
    pub full: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(vmid: VmidInt) -> Self {
        Self {
            vmid,
            acpi: ::std::default::Default::default(),
            affinity: ::std::default::Default::default(),
            agent: ::std::default::Default::default(),
            allow_ksm: ::std::default::Default::default(),
            amd_sev: ::std::default::Default::default(),
            arch: ::std::default::Default::default(),
            archive: ::std::default::Default::default(),
            args: ::std::default::Default::default(),
            audio0: ::std::default::Default::default(),
            autostart: ::std::default::Default::default(),
            balloon: ::std::default::Default::default(),
            bios: ::std::default::Default::default(),
            boot: ::std::default::Default::default(),
            bootdisk: ::std::default::Default::default(),
            bwlimit: ::std::default::Default::default(),
            cdrom: ::std::default::Default::default(),
            cicustom: ::std::default::Default::default(),
            cipassword: ::std::default::Default::default(),
            citype: ::std::default::Default::default(),
            ciupgrade: ::std::default::Default::default(),
            ciuser: ::std::default::Default::default(),
            cores: ::std::default::Default::default(),
            cpu: ::std::default::Default::default(),
            cpulimit: ::std::default::Default::default(),
            cpuunits: ::std::default::Default::default(),
            description: ::std::default::Default::default(),
            efidisk0: ::std::default::Default::default(),
            force: ::std::default::Default::default(),
            freeze: ::std::default::Default::default(),
            ha_managed: ::std::default::Default::default(),
            hookscript: ::std::default::Default::default(),
            hostpcis: ::std::default::Default::default(),
            hotplug: ::std::default::Default::default(),
            hugepages: ::std::default::Default::default(),
            ides: ::std::default::Default::default(),
            import_working_storage: ::std::default::Default::default(),
            intel_tdx: ::std::default::Default::default(),
            ipconfigs: ::std::default::Default::default(),
            ivshmem: ::std::default::Default::default(),
            keephugepages: ::std::default::Default::default(),
            keyboard: ::std::default::Default::default(),
            kvm: ::std::default::Default::default(),
            live_restore: ::std::default::Default::default(),
            localtime: ::std::default::Default::default(),
            lock: ::std::default::Default::default(),
            machine: ::std::default::Default::default(),
            memory: ::std::default::Default::default(),
            migrate_downtime: ::std::default::Default::default(),
            migrate_speed: ::std::default::Default::default(),
            name: ::std::default::Default::default(),
            nameserver: ::std::default::Default::default(),
            nets: ::std::default::Default::default(),
            numa: ::std::default::Default::default(),
            numas: ::std::default::Default::default(),
            onboot: ::std::default::Default::default(),
            ostype: ::std::default::Default::default(),
            parallels: ::std::default::Default::default(),
            pool: ::std::default::Default::default(),
            protection: ::std::default::Default::default(),
            reboot: ::std::default::Default::default(),
            rng0: ::std::default::Default::default(),
            satas: ::std::default::Default::default(),
            scsis: ::std::default::Default::default(),
            scsihw: ::std::default::Default::default(),
            searchdomain: ::std::default::Default::default(),
            serials: ::std::default::Default::default(),
            shares: ::std::default::Default::default(),
            smbios1: ::std::default::Default::default(),
            smp: ::std::default::Default::default(),
            sockets: ::std::default::Default::default(),
            spice_enhancements: ::std::default::Default::default(),
            sshkeys: ::std::default::Default::default(),
            start: ::std::default::Default::default(),
            startdate: ::std::default::Default::default(),
            startup: ::std::default::Default::default(),
            storage: ::std::default::Default::default(),
            tablet: ::std::default::Default::default(),
            tags: ::std::default::Default::default(),
            tdf: ::std::default::Default::default(),
            template: ::std::default::Default::default(),
            tpmstate0: ::std::default::Default::default(),
            unique: ::std::default::Default::default(),
            unuseds: ::std::default::Default::default(),
            usbs: ::std::default::Default::default(),
            vcpus: ::std::default::Default::default(),
            vga: ::std::default::Default::default(),
            virtios: ::std::default::Default::default(),
            virtiofss: ::std::default::Default::default(),
            vmgenid: ::std::default::Default::default(),
            vmstatestorage: ::std::default::Default::default(),
            watchdog: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable ACPI."]
    #[doc = ""]
    pub acpi: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of host cores used to execute guest processes, for example: 0,5,8-11"]
    #[doc = ""]
    pub affinity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable communication with the QEMU Guest Agent and its properties."]
    #[doc = ""]
    pub agent: Option<String>,
    #[serde(rename = "allow-ksm")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow memory pages of this guest to be merged via KSM (Kernel Samepage Merging)."]
    #[doc = ""]
    pub allow_ksm: Option<bool>,
    #[serde(rename = "amd-sev")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Secure Encrypted Virtualization (SEV) features by AMD CPUs"]
    #[doc = ""]
    pub amd_sev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Virtual processor architecture. Defaults to the host."]
    #[doc = ""]
    pub arch: Option<Arch>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The backup archive. Either the file system path to a .tar or .vma file (use '-' to pipe data from stdin) or a proxmox storage backup volume identifier."]
    #[doc = ""]
    pub archive: Option<ArchiveStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Arbitrary arguments passed to kvm."]
    #[doc = ""]
    #[doc = "Arbitrary arguments passed to kvm, for example:"]
    #[doc = ""]
    #[doc = "args: -no-reboot -smbios 'type=0,vendor=FOO'"]
    #[doc = ""]
    #[doc = "NOTE: this option is for experts only."]
    #[doc = ""]
    pub args: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure a audio device, useful in combination with QXL/Spice."]
    #[doc = ""]
    pub audio0: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Automatic restart after crash (currently ignored)."]
    #[doc = ""]
    pub autostart: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Amount of target RAM for the VM in MiB. Using zero disables the ballon driver."]
    #[doc = ""]
    pub balloon: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Select BIOS implementation."]
    #[doc = ""]
    pub bios: Option<Bios>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify guest boot order. Use the 'order=' sub-property as usage with no key or 'legacy=' is deprecated."]
    #[doc = ""]
    pub boot: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable booting from specified disk. Deprecated: Use 'boot: order=foo;bar' instead."]
    #[doc = ""]
    pub bootdisk: Option<BootdiskStr>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Override I/O bandwidth limit (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "This is an alias for option -ide2"]
    #[doc = ""]
    pub cdrom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "cloud-init: Specify custom files to replace the automatically generated ones at start."]
    #[doc = ""]
    pub cicustom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "cloud-init: Password to assign the user. Using this is generally not recommended. Use ssh keys instead. Also note that older cloud-init versions do not support hashed passwords."]
    #[doc = ""]
    pub cipassword: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies the cloud-init configuration format. The default depends on the configured operating system type (`ostype`. We use the `nocloud` format for Linux, and `configdrive2` for windows."]
    #[doc = ""]
    pub citype: Option<Citype>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "cloud-init: do an automatic package upgrade after the first boot."]
    #[doc = ""]
    pub ciupgrade: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "cloud-init: User name to change ssh keys and password for instead of the image's configured default user."]
    #[doc = ""]
    pub ciuser: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_non_zero_pos_int_optional",
        deserialize_with = "crate::types::deserialize_non_zero_pos_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of cores per socket."]
    #[doc = ""]
    pub cores: Option<std::num::NonZeroU64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Emulated CPU type."]
    #[doc = ""]
    pub cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit of CPU usage."]
    #[doc = ""]
    #[doc = "Limit of CPU usage."]
    #[doc = ""]
    #[doc = "NOTE: If the computer has 2 CPUs, it has total of '2' CPU time. Value '0' indicates no CPU limit."]
    #[doc = ""]
    pub cpulimit: Option<CpulimitNum>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU weight for a VM, will be clamped to \\\\[1, 10000\\\\] in cgroup v2."]
    #[doc = ""]
    #[doc = "CPU weight for a VM. Argument is used in the kernel fair scheduler. The larger the number is, the more CPU time this VM gets. Number is relative to weights of all the other running VMs."]
    #[doc = ""]
    pub cpuunits: Option<CpuunitsInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the VM. Shown in the web-interface VM's summary. This is saved as comment inside the configuration file."]
    #[doc = ""]
    pub description: Option<DescriptionStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure a disk for storing EFI vars. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and that the default EFI vars are copied to the volume instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[doc = ""]
    pub efidisk0: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow to overwrite existing VM."]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Freeze CPU at startup (use 'c' monitor command to start execution)."]
    #[doc = ""]
    pub freeze: Option<bool>,
    #[serde(rename = "ha-managed")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add the VM as a HA resource after it was created."]
    #[doc = ""]
    pub ha_managed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Script that will be executed during various steps in the vms lifetime."]
    #[doc = ""]
    pub hookscript: Option<String>,
    #[serde(rename = "hostpci[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedHostpcis, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedHostpcis, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Map host PCI devices into guest."]
    #[doc = ""]
    #[doc = "Map host PCI devices into guest."]
    #[doc = ""]
    #[doc = "NOTE: This option allows direct access to host hardware. So it is no longer"]
    #[doc = ""]
    #[doc = "possible to migrate such machines - use with special care."]
    #[doc = ""]
    #[doc = "CAUTION: Experimental! User reported problems with this option."]
    #[doc = ""]
    pub hostpcis: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Selectively enable hotplug features. This is a comma separated list of hotplug features: 'network', 'disk', 'cpu', 'memory', 'usb' and 'cloudinit'. Use '0' to disable hotplug completely. Using '1' as value is an alias for the default `network,disk,usb`. USB hotplugging is possible for guests with machine version \\\\>= 7.1 and ostype l26 or windows \\\\> 7."]
    #[doc = ""]
    pub hotplug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enables hugepages memory."]
    #[doc = ""]
    #[doc = "Sets the size of hugepages in MiB. If the value is set to 'any' then 1 GiB hugepages will be used if possible, otherwise the size will fall back to 2 MiB."]
    #[doc = ""]
    pub hugepages: Option<Hugepages>,
    #[serde(rename = "ide[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedIdes, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedIdes, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Use volume as IDE hard disk or CD-ROM (n is 0 to 3). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[doc = ""]
    pub ides: ::std::collections::HashMap<u32, String>,
    #[serde(rename = "import-working-storage")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A file-based storage with 'images' content-type enabled, which is used as an intermediary extraction storage during import. Defaults to the source storage."]
    #[doc = ""]
    pub import_working_storage: Option<String>,
    #[serde(rename = "intel-tdx")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Trusted Domain Extension (TDX) features by Intel CPUs"]
    #[doc = ""]
    pub intel_tdx: Option<String>,
    #[serde(rename = "ipconfig[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedIpconfigs, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedIpconfigs, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "cloud-init: Specify IP addresses and gateways for the corresponding interface."]
    #[doc = ""]
    #[doc = "IP addresses use CIDR notation, gateways are optional but need an IP of the same type specified."]
    #[doc = ""]
    #[doc = "The special string 'dhcp' can be used for IP addresses to use DHCP, in which case no explicit"]
    #[doc = ""]
    #[doc = "gateway should be provided."]
    #[doc = ""]
    #[doc = "For IPv6 the special string 'auto' can be used to use stateless autoconfiguration. This requires"]
    #[doc = ""]
    #[doc = "cloud-init 19.4 or newer."]
    #[doc = ""]
    #[doc = "If cloud-init is enabled and neither an IPv4 nor an IPv6 address is specified, it defaults to using"]
    #[doc = ""]
    #[doc = "dhcp on IPv4."]
    #[doc = ""]
    pub ipconfigs: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Inter-VM shared memory. Useful for direct communication between VMs, or to the host."]
    #[doc = ""]
    pub ivshmem: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use together with hugepages. If enabled, hugepages will not not be deleted after VM shutdown and can be used for subsequent starts."]
    #[doc = ""]
    pub keephugepages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Keyboard layout for VNC server. This option is generally not required and is often better handled from within the guest OS."]
    #[doc = ""]
    pub keyboard: Option<Keyboard>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable KVM hardware virtualization."]
    #[doc = ""]
    pub kvm: Option<bool>,
    #[serde(rename = "live-restore")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Start the VM immediately while importing or restoring in the background."]
    #[doc = ""]
    pub live_restore: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set the real time clock (RTC) to local time. This is enabled by default if the `ostype` indicates a Microsoft Windows OS."]
    #[doc = ""]
    pub localtime: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Lock/unlock the VM."]
    #[doc = ""]
    pub lock: Option<Lock>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the QEMU machine."]
    #[doc = ""]
    pub machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Memory properties."]
    #[doc = ""]
    pub memory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set maximum tolerated downtime (in seconds) for migrations. Should the migration not be able to converge in the very end, because too much newly dirtied RAM needs to be transferred, the limit will be increased automatically step-by-step until migration can converge."]
    #[doc = ""]
    pub migrate_downtime: Option<MigrateDowntimeNum>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set maximum speed (in MB/s) for migrations. Value 0 is no limit."]
    #[doc = ""]
    pub migrate_speed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set a name for the VM. Only used on the configuration web interface."]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "cloud-init: Sets DNS server IP address for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set."]
    #[doc = ""]
    pub nameserver: Option<String>,
    #[serde(rename = "net[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedNets, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedNets, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Specify network devices."]
    #[doc = ""]
    pub nets: ::std::collections::HashMap<u32, String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable NUMA."]
    #[doc = ""]
    pub numa: Option<bool>,
    #[serde(rename = "numa[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedNumas, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedNumas, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "NUMA topology."]
    #[doc = ""]
    pub numas: ::std::collections::HashMap<u32, String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specifies whether a VM will be started during system bootup."]
    #[doc = ""]
    pub onboot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify guest operating system."]
    #[doc = ""]
    #[doc = "Specify guest operating system. This is used to enable special"]
    #[doc = ""]
    #[doc = "optimization/features for specific operating systems:"]
    #[doc = ""]
    #[doc = "\\\\[horizontal\\\\]"]
    #[doc = ""]
    #[doc = "other;; unspecified OS"]
    #[doc = ""]
    #[doc = "wxp;; Microsoft Windows XP"]
    #[doc = ""]
    #[doc = "w2k;; Microsoft Windows 2000"]
    #[doc = ""]
    #[doc = "w2k3;; Microsoft Windows 2003"]
    #[doc = ""]
    #[doc = "w2k8;; Microsoft Windows 2008"]
    #[doc = ""]
    #[doc = "wvista;; Microsoft Windows Vista"]
    #[doc = ""]
    #[doc = "win7;; Microsoft Windows 7"]
    #[doc = ""]
    #[doc = "win8;; Microsoft Windows 8/2012/2012r2"]
    #[doc = ""]
    #[doc = "win10;; Microsoft Windows 10/2016/2019"]
    #[doc = ""]
    #[doc = "win11;; Microsoft Windows 11/2022/2025"]
    #[doc = ""]
    #[doc = "l24;; Linux 2.4 Kernel"]
    #[doc = ""]
    #[doc = "l26;; Linux 2.6 - 6.X Kernel"]
    #[doc = ""]
    #[doc = "solaris;; Solaris/OpenSolaris/OpenIndiania kernel"]
    #[doc = ""]
    pub ostype: Option<Ostype>,
    #[serde(rename = "parallel[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedParallels, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedParallels, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Map host parallel devices (n is 0 to 2)."]
    #[doc = ""]
    #[doc = "Map host parallel devices (n is 0 to 2)."]
    #[doc = ""]
    #[doc = "NOTE: This option allows direct access to host hardware. So it is no longer possible to migrate such"]
    #[doc = ""]
    #[doc = "machines - use with special care."]
    #[doc = ""]
    #[doc = "CAUTION: Experimental! User reported problems with this option."]
    #[doc = ""]
    pub parallels: ::std::collections::HashMap<u32, ParallelNStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Add the VM to the specified pool."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Sets the protection flag of the VM. This will disable the remove VM and remove disk operations."]
    #[doc = ""]
    pub protection: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow reboot. If set to '0' the VM exit on reboot."]
    #[doc = ""]
    pub reboot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure a VirtIO-based Random Number Generator."]
    #[doc = ""]
    pub rng0: Option<String>,
    #[serde(rename = "sata[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedSatas, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedSatas, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Use volume as SATA hard disk or CD-ROM (n is 0 to 5). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[doc = ""]
    pub satas: ::std::collections::HashMap<u32, String>,
    #[serde(rename = "scsi[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedScsis, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedScsis, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Use volume as SCSI hard disk or CD-ROM (n is 0 to 30). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[doc = ""]
    pub scsis: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "SCSI controller model"]
    #[doc = ""]
    pub scsihw: Option<Scsihw>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "cloud-init: Sets DNS search domains for a container. Create will automatically use the setting from the host if neither searchdomain nor nameserver are set."]
    #[doc = ""]
    pub searchdomain: Option<String>,
    #[serde(rename = "serial[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedSerials, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedSerials, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Create a serial device inside the VM (n is 0 to 3)"]
    #[doc = ""]
    #[doc = "Create a serial device inside the VM (n is 0 to 3), and pass through a"]
    #[doc = ""]
    #[doc = "host serial device (i.e. /dev/ttyS0), or create a unix socket on the"]
    #[doc = ""]
    #[doc = "host side (use 'qm terminal' to open a terminal connection)."]
    #[doc = ""]
    #[doc = "NOTE: If you pass through a host serial device, it is no longer possible to migrate such machines -"]
    #[doc = ""]
    #[doc = "use with special care."]
    #[doc = ""]
    #[doc = "CAUTION: Experimental! User reported problems with this option."]
    #[doc = ""]
    pub serials: ::std::collections::HashMap<u32, SerialNStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Amount of memory shares for auto-ballooning. The larger the number is, the more memory this VM gets. Number is relative to weights of all other running VMs. Using zero disables auto-ballooning. Auto-ballooning is done by pvestatd."]
    #[doc = ""]
    pub shares: Option<SharesInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify SMBIOS type 1 fields."]
    #[doc = ""]
    pub smbios1: Option<Smbios1Str>,
    #[serde(
        serialize_with = "crate::types::serialize_non_zero_pos_int_optional",
        deserialize_with = "crate::types::deserialize_non_zero_pos_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of CPUs. Please use option -sockets instead."]
    #[doc = ""]
    pub smp: Option<std::num::NonZeroU64>,
    #[serde(
        serialize_with = "crate::types::serialize_non_zero_pos_int_optional",
        deserialize_with = "crate::types::deserialize_non_zero_pos_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The number of CPU sockets."]
    #[doc = ""]
    pub sockets: Option<std::num::NonZeroU64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure additional enhancements for SPICE."]
    #[doc = ""]
    pub spice_enhancements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "cloud-init: Setup public SSH keys (one key per line, OpenSSH format)."]
    #[doc = ""]
    pub sshkeys: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Start VM after it was created successfully."]
    #[doc = ""]
    pub start: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set the initial date of the real time clock. Valid format for date are:'now' or '2006-06-17T16:01:21' or '2006-06-17'."]
    #[doc = ""]
    pub startdate: Option<StartdateStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Startup and shutdown behavior. Order is a non-negative number defining the general startup order. Shutdown in done with reverse ordering. Additionally you can set the 'up' or 'down' delay in seconds, which specifies a delay to wait before the next VM is started or stopped."]
    #[doc = ""]
    pub startup: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default storage."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable the USB tablet device."]
    #[doc = ""]
    #[doc = "Enable/disable the USB tablet device. This device is usually needed to allow absolute mouse positioning with VNC. Else the mouse runs out of sync with normal VNC clients. If you're running lots of console-only guests on one host, you may consider disabling this to save some context switches. This is turned off by default if you use spice (`qm set \\\\<vmid\\\\> --vga qxl`)."]
    #[doc = ""]
    pub tablet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Tags of the VM. This is only meta information."]
    #[doc = ""]
    pub tags: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable time drift fix."]
    #[doc = ""]
    pub tdf: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable Template."]
    #[doc = ""]
    pub template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure a Disk for storing TPM state. The format is fixed to 'raw'. Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Note that SIZE_IN_GiB is ignored here and 4 MiB will be used instead. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[doc = ""]
    pub tpmstate0: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Assign a unique random ethernet address."]
    #[doc = ""]
    pub unique: Option<bool>,
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
    #[serde(rename = "usb[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedUsbs, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedUsbs, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Configure an USB device (n is 0 to 4, for machine version \\\\>= 7.1 and ostype l26 or windows \\\\> 7, n can be up to 14)."]
    #[doc = ""]
    pub usbs: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of hotplugged vcpus."]
    #[doc = ""]
    pub vcpus: Option<VcpusInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Configure the VGA hardware."]
    #[doc = ""]
    #[doc = "Configure the VGA Hardware. If you want to use high resolution modes (\\\\>= 1280x1024x16) you may need to increase the vga memory option. Since QEMU 2.9 the default VGA display type is 'std' for all OS types besides some Windows versions (XP and older) which use 'cirrus'. The 'qxl' option enables the SPICE display server. For win* OS you can select how many independent displays you want, Linux guests can add displays them self."]
    #[doc = ""]
    #[doc = "You can also run without any graphic card, using a serial device as terminal."]
    #[doc = ""]
    pub vga: Option<String>,
    #[serde(rename = "virtio[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedVirtios, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedVirtios, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Use volume as VIRTIO hard disk (n is 0 to 15). Use the special syntax STORAGE_ID:SIZE_IN_GiB to allocate a new volume. Use STORAGE_ID:0 and the 'import-from' parameter to import from an existing volume."]
    #[doc = ""]
    pub virtios: ::std::collections::HashMap<u32, String>,
    #[serde(rename = "virtiofs[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedVirtiofss, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedVirtiofss, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Configuration for sharing a directory between host and guest using Virtio-fs."]
    #[doc = ""]
    pub virtiofss: ::std::collections::HashMap<u32, String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set VM Generation ID. Use '1' to autogenerate on create or update, pass '0' to disable explicitly."]
    #[doc = ""]
    #[doc = "The VM generation ID (vmgenid) device exposes a 128-bit integer value identifier to the guest OS. This allows to notify the guest operating system when the virtual machine is executed with a different configuration (e.g. snapshot execution or creation from a template). The guest operating system notices the change, and is then able to react as appropriate by marking its copies of distributed databases as dirty, re-initializing its random number generator, etc."]
    #[doc = ""]
    #[doc = "Note that auto-creation only works when done through API/CLI create or update methods, but not when manually editing the config file."]
    #[doc = ""]
    pub vmgenid: Option<VmgenidStr>,
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub vmid: VmidInt,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default storage for VM state volumes/files."]
    #[doc = ""]
    pub vmstatestorage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create a virtual hardware watchdog device."]
    #[doc = ""]
    #[doc = "Create a virtual hardware watchdog device. Once enabled (by a guest action), the watchdog must be periodically polled by an agent inside the guest or else the watchdog will reset the guest (or execute the respective action specified)"]
    #[doc = ""]
    pub watchdog: Option<String>,
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
                <NumberedHostpcis as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedIdes as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedIpconfigs as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedNets as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedNumas as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedParallels as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedSatas as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedScsis as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedSerials as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedUnuseds as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedUsbs as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedVirtios as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
                <NumberedVirtiofss as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
            ];
            array.iter().any(|f| f(input))
        }
        the_test as _
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Virtual processor architecture. Defaults to the host."]
#[doc = ""]
pub enum Arch {
    #[serde(rename = "aarch64")]
    Aarch64,
    #[serde(rename = "x86_64")]
    X8664,
}
impl TryFrom<&str> for Arch {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "aarch64" => Ok(Self::Aarch64),
            "x86_64" => Ok(Self::X8664),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Select BIOS implementation."]
#[doc = ""]
pub enum Bios {
    #[serde(rename = "ovmf")]
    Ovmf,
    #[serde(rename = "seabios")]
    #[default]
    Seabios,
}
impl TryFrom<&str> for Bios {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ovmf" => Ok(Self::Ovmf),
            "seabios" => Ok(Self::Seabios),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Specifies the cloud-init configuration format. The default depends on the configured operating system type (`ostype`. We use the `nocloud` format for Linux, and `configdrive2` for windows."]
#[doc = ""]
pub enum Citype {
    #[serde(rename = "configdrive2")]
    Configdrive2,
    #[serde(rename = "nocloud")]
    Nocloud,
    #[serde(rename = "opennebula")]
    Opennebula,
}
impl TryFrom<&str> for Citype {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "configdrive2" => Ok(Self::Configdrive2),
            "nocloud" => Ok(Self::Nocloud),
            "opennebula" => Ok(Self::Opennebula),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Enables hugepages memory."]
#[doc = ""]
#[doc = "Sets the size of hugepages in MiB. If the value is set to 'any' then 1 GiB hugepages will be used if possible, otherwise the size will fall back to 2 MiB."]
#[doc = ""]
pub enum Hugepages {
    #[serde(rename = "1024")]
    _1024,
    #[serde(rename = "2")]
    _2,
    #[serde(rename = "any")]
    Any,
}
impl TryFrom<&str> for Hugepages {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "1024" => Ok(Self::_1024),
            "2" => Ok(Self::_2),
            "any" => Ok(Self::Any),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Keyboard layout for VNC server. This option is generally not required and is often better handled from within the guest OS."]
#[doc = ""]
pub enum Keyboard {
    #[serde(rename = "da")]
    Da,
    #[serde(rename = "de")]
    De,
    #[serde(rename = "de-ch")]
    DeCh,
    #[serde(rename = "en-gb")]
    EnGb,
    #[serde(rename = "en-us")]
    EnUs,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "fi")]
    Fi,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "fr-be")]
    FrBe,
    #[serde(rename = "fr-ca")]
    FrCa,
    #[serde(rename = "fr-ch")]
    FrCh,
    #[serde(rename = "hu")]
    Hu,
    #[serde(rename = "is")]
    Is,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "mk")]
    Mk,
    #[serde(rename = "nl")]
    Nl,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "pl")]
    Pl,
    #[serde(rename = "pt")]
    Pt,
    #[serde(rename = "pt-br")]
    PtBr,
    #[serde(rename = "sl")]
    Sl,
    #[serde(rename = "sv")]
    Sv,
    #[serde(rename = "tr")]
    Tr,
}
impl TryFrom<&str> for Keyboard {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "da" => Ok(Self::Da),
            "de" => Ok(Self::De),
            "de-ch" => Ok(Self::DeCh),
            "en-gb" => Ok(Self::EnGb),
            "en-us" => Ok(Self::EnUs),
            "es" => Ok(Self::Es),
            "fi" => Ok(Self::Fi),
            "fr" => Ok(Self::Fr),
            "fr-be" => Ok(Self::FrBe),
            "fr-ca" => Ok(Self::FrCa),
            "fr-ch" => Ok(Self::FrCh),
            "hu" => Ok(Self::Hu),
            "is" => Ok(Self::Is),
            "it" => Ok(Self::It),
            "ja" => Ok(Self::Ja),
            "lt" => Ok(Self::Lt),
            "mk" => Ok(Self::Mk),
            "nl" => Ok(Self::Nl),
            "no" => Ok(Self::No),
            "pl" => Ok(Self::Pl),
            "pt" => Ok(Self::Pt),
            "pt-br" => Ok(Self::PtBr),
            "sl" => Ok(Self::Sl),
            "sv" => Ok(Self::Sv),
            "tr" => Ok(Self::Tr),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Lock/unlock the VM."]
#[doc = ""]
pub enum Lock {
    #[serde(rename = "backup")]
    Backup,
    #[serde(rename = "clone")]
    Clone,
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "migrate")]
    Migrate,
    #[serde(rename = "rollback")]
    Rollback,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "snapshot-delete")]
    SnapshotDelete,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "suspending")]
    Suspending,
}
impl TryFrom<&str> for Lock {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "backup" => Ok(Self::Backup),
            "clone" => Ok(Self::Clone),
            "create" => Ok(Self::Create),
            "migrate" => Ok(Self::Migrate),
            "rollback" => Ok(Self::Rollback),
            "snapshot" => Ok(Self::Snapshot),
            "snapshot-delete" => Ok(Self::SnapshotDelete),
            "suspended" => Ok(Self::Suspended),
            "suspending" => Ok(Self::Suspending),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Specify guest operating system."]
#[doc = ""]
#[doc = "Specify guest operating system. This is used to enable special"]
#[doc = ""]
#[doc = "optimization/features for specific operating systems:"]
#[doc = ""]
#[doc = "\\[horizontal\\]"]
#[doc = ""]
#[doc = "other;; unspecified OS"]
#[doc = ""]
#[doc = "wxp;; Microsoft Windows XP"]
#[doc = ""]
#[doc = "w2k;; Microsoft Windows 2000"]
#[doc = ""]
#[doc = "w2k3;; Microsoft Windows 2003"]
#[doc = ""]
#[doc = "w2k8;; Microsoft Windows 2008"]
#[doc = ""]
#[doc = "wvista;; Microsoft Windows Vista"]
#[doc = ""]
#[doc = "win7;; Microsoft Windows 7"]
#[doc = ""]
#[doc = "win8;; Microsoft Windows 8/2012/2012r2"]
#[doc = ""]
#[doc = "win10;; Microsoft Windows 10/2016/2019"]
#[doc = ""]
#[doc = "win11;; Microsoft Windows 11/2022/2025"]
#[doc = ""]
#[doc = "l24;; Linux 2.4 Kernel"]
#[doc = ""]
#[doc = "l26;; Linux 2.6 - 6.X Kernel"]
#[doc = ""]
#[doc = "solaris;; Solaris/OpenSolaris/OpenIndiania kernel"]
#[doc = ""]
pub enum Ostype {
    #[serde(rename = "l24")]
    L24,
    #[serde(rename = "l26")]
    L26,
    #[serde(rename = "other")]
    #[default]
    Other,
    #[serde(rename = "solaris")]
    Solaris,
    #[serde(rename = "w2k")]
    W2k,
    #[serde(rename = "w2k3")]
    W2k3,
    #[serde(rename = "w2k8")]
    W2k8,
    #[serde(rename = "win10")]
    Win10,
    #[serde(rename = "win11")]
    Win11,
    #[serde(rename = "win7")]
    Win7,
    #[serde(rename = "win8")]
    Win8,
    #[serde(rename = "wvista")]
    Wvista,
    #[serde(rename = "wxp")]
    Wxp,
}
impl TryFrom<&str> for Ostype {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "l24" => Ok(Self::L24),
            "l26" => Ok(Self::L26),
            "other" => Ok(Self::Other),
            "solaris" => Ok(Self::Solaris),
            "w2k" => Ok(Self::W2k),
            "w2k3" => Ok(Self::W2k3),
            "w2k8" => Ok(Self::W2k8),
            "win10" => Ok(Self::Win10),
            "win11" => Ok(Self::Win11),
            "win7" => Ok(Self::Win7),
            "win8" => Ok(Self::Win8),
            "wvista" => Ok(Self::Wvista),
            "wxp" => Ok(Self::Wxp),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "SCSI controller model"]
#[doc = ""]
pub enum Scsihw {
    #[serde(rename = "lsi")]
    #[default]
    Lsi,
    #[serde(rename = "lsi53c810")]
    Lsi53c810,
    #[serde(rename = "megasas")]
    Megasas,
    #[serde(rename = "pvscsi")]
    Pvscsi,
    #[serde(rename = "virtio-scsi-pci")]
    VirtioScsiPci,
    #[serde(rename = "virtio-scsi-single")]
    VirtioScsiSingle,
}
impl TryFrom<&str> for Scsihw {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "lsi" => Ok(Self::Lsi),
            "lsi53c810" => Ok(Self::Lsi53c810),
            "megasas" => Ok(Self::Megasas),
            "pvscsi" => Ok(Self::Pvscsi),
            "virtio-scsi-pci" => Ok(Self::VirtioScsiPci),
            "virtio-scsi-single" => Ok(Self::VirtioScsiSingle),
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
#[derive(Default)]
struct NumberedHostpcis;
impl crate::types::multi::NumberedItems for NumberedHostpcis {
    type Item = String;
    const PREFIX: &'static str = "hostpci";
}
#[derive(Default)]
struct NumberedIdes;
impl crate::types::multi::NumberedItems for NumberedIdes {
    type Item = String;
    const PREFIX: &'static str = "ide";
}
#[derive(Default)]
struct NumberedIpconfigs;
impl crate::types::multi::NumberedItems for NumberedIpconfigs {
    type Item = String;
    const PREFIX: &'static str = "ipconfig";
}
#[derive(Default)]
struct NumberedNets;
impl crate::types::multi::NumberedItems for NumberedNets {
    type Item = String;
    const PREFIX: &'static str = "net";
}
#[derive(Default)]
struct NumberedNumas;
impl crate::types::multi::NumberedItems for NumberedNumas {
    type Item = String;
    const PREFIX: &'static str = "numa";
}
#[derive(Default)]
struct NumberedParallels;
impl crate::types::multi::NumberedItems for NumberedParallels {
    type Item = ParallelNStr;
    const PREFIX: &'static str = "parallel";
}
#[derive(Default)]
struct NumberedSatas;
impl crate::types::multi::NumberedItems for NumberedSatas {
    type Item = String;
    const PREFIX: &'static str = "sata";
}
#[derive(Default)]
struct NumberedScsis;
impl crate::types::multi::NumberedItems for NumberedScsis {
    type Item = String;
    const PREFIX: &'static str = "scsi";
}
#[derive(Default)]
struct NumberedSerials;
impl crate::types::multi::NumberedItems for NumberedSerials {
    type Item = SerialNStr;
    const PREFIX: &'static str = "serial";
}
#[derive(Default)]
struct NumberedUnuseds;
impl crate::types::multi::NumberedItems for NumberedUnuseds {
    type Item = String;
    const PREFIX: &'static str = "unused";
}
#[derive(Default)]
struct NumberedUsbs;
impl crate::types::multi::NumberedItems for NumberedUsbs {
    type Item = String;
    const PREFIX: &'static str = "usb";
}
#[derive(Default)]
struct NumberedVirtiofss;
impl crate::types::multi::NumberedItems for NumberedVirtiofss {
    type Item = String;
    const PREFIX: &'static str = "virtiofs";
}
#[derive(Default)]
struct NumberedVirtios;
impl crate::types::multi::NumberedItems for NumberedVirtios {
    type Item = String;
    const PREFIX: &'static str = "virtio";
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CpuunitsInt(i128);
impl crate::types::bounded_integer::BoundedInteger for CpuunitsInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(262144i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 262144";
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
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CpuunitsInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SharesInt(i128);
impl crate::types::bounded_integer::BoundedInteger for SharesInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = Some(50000i128);
    const DEFAULT: Option<i128> = Some(1000i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 0 and 50000";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for SharesInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for SharesInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SharesInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VcpusInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VcpusInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(0i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 1";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VcpusInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VcpusInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VcpusInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
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
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VmidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CpulimitNum(f64);
impl crate::types::bounded_number::BoundedNumber for CpulimitNum {
    const MIN: Option<f64> = Some(0f64);
    const MAX: Option<f64> = Some(128f64);
    const DEFAULT: Option<f64> = Some(0f64);
    const TYPE_DESCRIPTION: &'static str = "an number between 0 and 128";
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
        crate::types::bounded_number::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CpulimitNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_number::deserialize_bounded_number(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MigrateDowntimeNum(f64);
impl crate::types::bounded_number::BoundedNumber for MigrateDowntimeNum {
    const MIN: Option<f64> = Some(0f64);
    const MAX: Option<f64> = None::<f64>;
    const DEFAULT: Option<f64> = Some(0.1f64);
    const TYPE_DESCRIPTION: &'static str = "an number greater than or equal to 0";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for MigrateDowntimeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for MigrateDowntimeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for MigrateDowntimeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for MigrateDowntimeNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for MigrateDowntimeNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_number::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MigrateDowntimeNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_number::deserialize_bounded_number(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ArchiveStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for ArchiveStr {
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
impl std::convert::TryFrom<String> for ArchiveStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for ArchiveStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ArchiveStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BootdiskStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for BootdiskStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("(ide|sata|scsi|virtio)\\d+");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"(ide|sata|scsi|virtio)\\d+\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for BootdiskStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for BootdiskStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for BootdiskStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
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
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DescriptionStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ParallelNStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for ParallelNStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("/dev/parport\\d+|/dev/usb/lp\\d+");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"/dev/parport\\d+|/dev/usb/lp\\d+\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for ParallelNStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for ParallelNStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ParallelNStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SerialNStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for SerialNStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("(/dev/.+|socket)");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"(/dev/.+|socket)\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for SerialNStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for SerialNStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SerialNStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Smbios1Str {
    value: String,
}
impl crate::types::bounded_string::BoundedString for Smbios1Str {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(512usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 512";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for Smbios1Str {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for Smbios1Str {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for Smbios1Str {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StartdateStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for StartdateStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = Some("now");
    const PATTERN: Option<&'static str> =
        Some("(now|\\d{4}-\\d{1,2}-\\d{1,2}(T\\d{1,2}:\\d{1,2}:\\d{1,2})?)");
    const TYPE_DESCRIPTION: &'static str = "a string with pattern r\"(now|\\d{4}-\\d{1,2}-\\d{1,2}(T\\d{1,2}:\\d{1,2}:\\d{1,2})?)\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for StartdateStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for StartdateStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for StartdateStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct VmgenidStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for VmgenidStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = Some("1 (autogenerated)");
    const PATTERN: Option<&'static str> =
        Some("(?:[a-fA-F0-9]{8}(?:-[a-fA-F0-9]{4}){3}-[a-fA-F0-9]{12}|[01])");
    const TYPE_DESCRIPTION: &'static str = "a string with pattern r\"(?:[a-fA-F0-9]{8}(?:-[a-fA-F0-9]{4}){3}-[a-fA-F0-9]{12}|[01])\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for VmgenidStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for VmgenidStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VmgenidStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
impl<T> QemuClient<T>
where
    T: crate::client::Client,
{
    pub fn vmid(&self, vmid: crate::types::VmId) -> vmid::VmidClient<T> {
        vmid::VmidClient::<T>::new(self.client.clone(), &self.path, vmid)
    }
}
