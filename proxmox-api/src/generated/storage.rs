pub mod storage;
#[derive(Debug, Clone)]
pub struct StorageClient<T> {
    client: T,
    path: String,
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "/storage".to_string(),
        }
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Storage index."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new storage."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct ConfigPostOutputConfig {
    #[serde(rename = "encryption-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The, possibly auto-generated, encryption-key."]
    #[doc = ""]
    pub encryption_key: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(storage: String) -> Self {
        Self {
            storage,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub storage: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list storage of specific type"]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostOutput {
    pub fn new(storage: String, ty: Type) -> Self {
        Self {
            storage,
            ty,
            config: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Partial, possibly server generated, configuration properties."]
    #[doc = ""]
    pub config: Option<ConfigPostOutputConfig>,
    #[doc = "The ID of the created storage."]
    #[doc = ""]
    pub storage: String,
    #[serde(rename = "type")]
    #[doc = "The type of the created storage."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(storage: String, ty: Type) -> Self {
        Self {
            storage,
            ty,
            authsupported: ::std::default::Default::default(),
            base: ::std::default::Default::default(),
            blocksize: ::std::default::Default::default(),
            bwlimit: ::std::default::Default::default(),
            comstar_hg: ::std::default::Default::default(),
            comstar_tg: ::std::default::Default::default(),
            content: ::std::default::Default::default(),
            content_dirs: ::std::default::Default::default(),
            create_base_path: ::std::default::Default::default(),
            create_subdirs: ::std::default::Default::default(),
            data_pool: ::std::default::Default::default(),
            datastore: ::std::default::Default::default(),
            disable: ::std::default::Default::default(),
            domain: ::std::default::Default::default(),
            encryption_key: ::std::default::Default::default(),
            export: ::std::default::Default::default(),
            fingerprint: ::std::default::Default::default(),
            format: ::std::default::Default::default(),
            fs_name: ::std::default::Default::default(),
            fuse: ::std::default::Default::default(),
            is_mountpoint: ::std::default::Default::default(),
            iscsiprovider: ::std::default::Default::default(),
            keyring: ::std::default::Default::default(),
            krbd: ::std::default::Default::default(),
            lio_tpg: ::std::default::Default::default(),
            master_pubkey: ::std::default::Default::default(),
            max_protected_backups: ::std::default::Default::default(),
            mkdir: ::std::default::Default::default(),
            monhost: ::std::default::Default::default(),
            mountpoint: ::std::default::Default::default(),
            namespace: ::std::default::Default::default(),
            nocow: ::std::default::Default::default(),
            nodes: ::std::default::Default::default(),
            nowritecache: ::std::default::Default::default(),
            options: ::std::default::Default::default(),
            password: ::std::default::Default::default(),
            path: ::std::default::Default::default(),
            pool: ::std::default::Default::default(),
            port: ::std::default::Default::default(),
            portal: ::std::default::Default::default(),
            preallocation: ::std::default::Default::default(),
            prune_backups: ::std::default::Default::default(),
            saferemove: ::std::default::Default::default(),
            saferemove_stepsize: ::std::default::Default::default(),
            saferemove_throughput: ::std::default::Default::default(),
            server: ::std::default::Default::default(),
            share: ::std::default::Default::default(),
            shared: ::std::default::Default::default(),
            skip_cert_verification: ::std::default::Default::default(),
            smbversion: ::std::default::Default::default(),
            snapshot_as_volume_chain: ::std::default::Default::default(),
            sparse: ::std::default::Default::default(),
            subdir: ::std::default::Default::default(),
            tagged_only: ::std::default::Default::default(),
            target: ::std::default::Default::default(),
            thinpool: ::std::default::Default::default(),
            username: ::std::default::Default::default(),
            vgname: ::std::default::Default::default(),
            zfs_base_path: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Authsupported."]
    #[doc = ""]
    pub authsupported: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Base volume. This volume is automatically activated."]
    #[doc = ""]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "block size"]
    #[doc = ""]
    pub blocksize: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set I/O bandwidth limit for various operations (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "host group for comstar views"]
    #[doc = ""]
    pub comstar_hg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "target group for comstar views"]
    #[doc = ""]
    pub comstar_tg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allowed content types."]
    #[doc = ""]
    #[doc = "NOTE: the value 'rootdir' is used for Containers, and value 'images' for VMs."]
    #[doc = ""]
    pub content: Option<String>,
    #[serde(rename = "content-dirs")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Overrides for default content type directories."]
    #[doc = ""]
    pub content_dirs: Option<String>,
    #[serde(rename = "create-base-path")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create the base directory if it doesn't exist."]
    #[doc = ""]
    pub create_base_path: Option<bool>,
    #[serde(rename = "create-subdirs")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Populate the directory with the default structure."]
    #[doc = ""]
    pub create_subdirs: Option<bool>,
    #[serde(rename = "data-pool")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Data Pool (for erasure coding only)"]
    #[doc = ""]
    pub data_pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Proxmox Backup Server datastore name."]
    #[doc = ""]
    pub datastore: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Flag to disable the storage."]
    #[doc = ""]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CIFS domain."]
    #[doc = ""]
    pub domain: Option<DomainStr>,
    #[serde(rename = "encryption-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Encryption key. Use 'autogen' to generate one automatically without passphrase."]
    #[doc = ""]
    pub encryption_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "NFS export path."]
    #[doc = ""]
    pub export: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    #[doc = ""]
    pub fingerprint: Option<FingerprintStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default image format."]
    #[doc = ""]
    pub format: Option<Format>,
    #[serde(rename = "fs-name")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The Ceph filesystem name."]
    #[doc = ""]
    pub fs_name: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mount CephFS through FUSE."]
    #[doc = ""]
    pub fuse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Assume the given path is an externally managed mountpoint and consider the storage offline if it is not mounted. Using a boolean (yes/no) value serves as a shortcut to using the target path in this field."]
    #[doc = ""]
    pub is_mountpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "iscsi provider"]
    #[doc = ""]
    pub iscsiprovider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Client keyring contents (for external clusters)."]
    #[doc = ""]
    pub keyring: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Always access rbd through krbd kernel module."]
    #[doc = ""]
    pub krbd: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "target portal group for Linux LIO targets"]
    #[doc = ""]
    pub lio_tpg: Option<String>,
    #[serde(rename = "master-pubkey")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Base64-encoded, PEM-formatted public RSA key. Used to encrypt a copy of the encryption-key which will be added to each encrypted backup."]
    #[doc = ""]
    pub master_pubkey: Option<String>,
    #[serde(rename = "max-protected-backups")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of protected backups per guest. Use '-1' for unlimited."]
    #[doc = ""]
    pub max_protected_backups: Option<MaxProtectedBackupsInt>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create the directory if it doesn't exist and populate it with default sub-dirs. NOTE: Deprecated, use the 'create-base-path' and 'create-subdirs' options instead."]
    #[doc = ""]
    pub mkdir: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP addresses of monitors (for external clusters)."]
    #[doc = ""]
    pub monhost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "mount point"]
    #[doc = ""]
    pub mountpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Namespace."]
    #[doc = ""]
    pub namespace: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set the NOCOW flag on files. Disables data checksumming and causes data errors to be unrecoverable from while allowing direct I/O. Only use this if data does not need to be any more safe than on a single ext4 formatted disk with no underlying raid system."]
    #[doc = ""]
    pub nocow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of nodes for which the storage configuration applies."]
    #[doc = ""]
    pub nodes: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "disable write caching on the target"]
    #[doc = ""]
    pub nowritecache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "NFS/CIFS mount options (see 'man nfs' or 'man mount.cifs')"]
    #[doc = ""]
    pub options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Password for accessing the share/datastore."]
    #[doc = ""]
    pub password: Option<PasswordStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "File system path."]
    #[doc = ""]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pool."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use this port to connect to the storage instead of the default one (for example, with PBS or ESXi). For NFS and CIFS, use the 'options' option to configure the port via the mount options."]
    #[doc = ""]
    pub port: Option<PortInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "iSCSI portal (IP or DNS name with optional port)."]
    #[doc = ""]
    pub portal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Preallocation mode for raw and qcow2 images. Using 'metadata' on raw images results in preallocation=off."]
    #[doc = ""]
    pub preallocation: Option<Preallocation>,
    #[serde(rename = "prune-backups")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The retention options with shorter intervals are processed first with --keep-last being the very first one. Each option covers a specific period of time. We say that backups within this period are covered by this option. The next option does not take care of already covered backups and only considers older backups."]
    #[doc = ""]
    pub prune_backups: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Zero-out data when removing LVs."]
    #[doc = ""]
    pub saferemove: Option<bool>,
    #[serde(rename = "saferemove-stepsize")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Wipe step size in MiB. It will be capped to the maximum supported by the storage."]
    #[doc = ""]
    pub saferemove_stepsize: Option<SaferemoveStepsizeInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Wipe throughput (cstream -t parameter value)."]
    #[doc = ""]
    pub saferemove_throughput: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Server IP or DNS name."]
    #[doc = ""]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CIFS share."]
    #[doc = ""]
    pub share: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Indicate that this is a single storage with the same contents on all nodes (or all listed in the 'nodes' option). It will not make the contents of a local storage automatically accessible to other nodes, it just marks an already shared storage as such!"]
    #[doc = ""]
    pub shared: Option<bool>,
    #[serde(rename = "skip-cert-verification")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable TLS certificate verification, only enable on fully trusted networks!"]
    #[doc = ""]
    pub skip_cert_verification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "SMB protocol version. 'default' if not set, negotiates the highest SMB2+ version supported by both the client and server."]
    #[doc = ""]
    pub smbversion: Option<Smbversion>,
    #[serde(rename = "snapshot-as-volume-chain")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable support for creating storage-vendor agnostic snapshot through volume backing-chains."]
    #[doc = ""]
    pub snapshot_as_volume_chain: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use sparse volumes"]
    #[doc = ""]
    pub sparse: Option<bool>,
    #[doc = "The storage identifier."]
    #[doc = ""]
    pub storage: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Subdir to mount."]
    #[doc = ""]
    pub subdir: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only use logical volumes tagged with 'pve-vm-ID'."]
    #[doc = ""]
    pub tagged_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "iSCSI target."]
    #[doc = ""]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LVM thin pool LV name."]
    #[doc = ""]
    pub thinpool: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Storage type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "RBD Id."]
    #[doc = ""]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Volume group name."]
    #[doc = ""]
    pub vgname: Option<String>,
    #[serde(rename = "zfs-base-path")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Base path where to look for the created ZFS block devices. Set automatically during creation if not specified. Usually '/dev/zvol'."]
    #[doc = ""]
    pub zfs_base_path: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Default image format."]
#[doc = ""]
pub enum Format {
    #[serde(rename = "qcow2")]
    Qcow2,
    #[serde(rename = "raw")]
    Raw,
    #[serde(rename = "subvol")]
    Subvol,
    #[serde(rename = "vmdk")]
    Vmdk,
}
impl TryFrom<&str> for Format {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "qcow2" => Ok(Self::Qcow2),
            "raw" => Ok(Self::Raw),
            "subvol" => Ok(Self::Subvol),
            "vmdk" => Ok(Self::Vmdk),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Preallocation mode for raw and qcow2 images. Using 'metadata' on raw images results in preallocation=off."]
#[doc = ""]
pub enum Preallocation {
    #[serde(rename = "falloc")]
    Falloc,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "metadata")]
    #[default]
    Metadata,
    #[serde(rename = "off")]
    Off,
}
impl TryFrom<&str> for Preallocation {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "falloc" => Ok(Self::Falloc),
            "full" => Ok(Self::Full),
            "metadata" => Ok(Self::Metadata),
            "off" => Ok(Self::Off),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "SMB protocol version. 'default' if not set, negotiates the highest SMB2+ version supported by both the client and server."]
#[doc = ""]
pub enum Smbversion {
    #[serde(rename = "2.0")]
    _20,
    #[serde(rename = "2.1")]
    _21,
    #[serde(rename = "3")]
    _3,
    #[serde(rename = "3.0")]
    _30,
    #[serde(rename = "3.11")]
    _311,
    #[serde(rename = "default")]
    #[default]
    Default,
}
impl TryFrom<&str> for Smbversion {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "2.0" => Ok(Self::_20),
            "2.1" => Ok(Self::_21),
            "3" => Ok(Self::_3),
            "3.0" => Ok(Self::_30),
            "3.11" => Ok(Self::_311),
            "default" => Ok(Self::Default),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Only list storage of specific type"]
#[doc = ""]
pub enum Type {
    #[serde(rename = "btrfs")]
    Btrfs,
    #[serde(rename = "cephfs")]
    Cephfs,
    #[serde(rename = "cifs")]
    Cifs,
    #[serde(rename = "dir")]
    Dir,
    #[serde(rename = "esxi")]
    Esxi,
    #[serde(rename = "iscsi")]
    Iscsi,
    #[serde(rename = "iscsidirect")]
    Iscsidirect,
    #[serde(rename = "lvm")]
    Lvm,
    #[serde(rename = "lvmthin")]
    Lvmthin,
    #[serde(rename = "nfs")]
    Nfs,
    #[serde(rename = "pbs")]
    Pbs,
    #[serde(rename = "rbd")]
    Rbd,
    #[serde(rename = "zfs")]
    Zfs,
    #[serde(rename = "zfspool")]
    Zfspool,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "btrfs" => Ok(Self::Btrfs),
            "cephfs" => Ok(Self::Cephfs),
            "cifs" => Ok(Self::Cifs),
            "dir" => Ok(Self::Dir),
            "esxi" => Ok(Self::Esxi),
            "iscsi" => Ok(Self::Iscsi),
            "iscsidirect" => Ok(Self::Iscsidirect),
            "lvm" => Ok(Self::Lvm),
            "lvmthin" => Ok(Self::Lvmthin),
            "nfs" => Ok(Self::Nfs),
            "pbs" => Ok(Self::Pbs),
            "rbd" => Ok(Self::Rbd),
            "zfs" => Ok(Self::Zfs),
            "zfspool" => Ok(Self::Zfspool),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxProtectedBackupsInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MaxProtectedBackupsInt {
    const MIN: Option<i128> = Some(-1i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to -1";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MaxProtectedBackupsInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MaxProtectedBackupsInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxProtectedBackupsInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PortInt(i128);
impl crate::types::bounded_integer::BoundedInteger for PortInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(65535i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 65535";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for PortInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for PortInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PortInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SaferemoveStepsizeInt(i128);
impl crate::types::bounded_integer::BoundedInteger for SaferemoveStepsizeInt {
    const MIN: Option<i128> = None::<i128>;
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(32i128);
    const TYPE_DESCRIPTION: &'static str = "a valid integer";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for SaferemoveStepsizeInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for SaferemoveStepsizeInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for SaferemoveStepsizeInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DomainStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DomainStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(256usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 256";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DomainStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DomainStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DomainStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FingerprintStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for FingerprintStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("([A-Fa-f0-9]{2}:){31}[A-Fa-f0-9]{2}");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"([A-Fa-f0-9]{2}:){31}[A-Fa-f0-9]{2}\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for FingerprintStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for FingerprintStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for FingerprintStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PasswordStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for PasswordStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(256usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 256";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for PasswordStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for PasswordStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PasswordStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn storage(&self, storage: &str) -> storage::StorageClient<T> {
        storage::StorageClient::<T>::new(self.client.clone(), &self.path, storage)
    }
}
