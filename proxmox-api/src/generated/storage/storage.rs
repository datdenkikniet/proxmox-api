#[derive(Debug, Clone)]
pub struct StorageClient<T> {
    client: T,
    path: String,
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, storage: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, storage),
        }
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete storage configuration."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read storage configuration."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update storage configuration."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<PutOutput, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct ConfigPutOutputConfig {
    #[serde(rename = "encryption-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The, possible auto-generated, encryption-key."]
    #[doc = ""]
    pub encryption_key: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PutOutput {
    pub fn new(storage: String, ty: Type) -> Self {
        Self {
            storage,
            ty,
            config: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Partial, possible server generated, configuration properties."]
    #[doc = ""]
    pub config: Option<ConfigPutOutputConfig>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
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
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
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
    pub domain: Option<String>,
    #[serde(rename = "encryption-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Encryption key. Use 'autogen' to generate one automatically without passphrase."]
    #[doc = ""]
    pub encryption_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    #[doc = ""]
    pub fingerprint: Option<String>,
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
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of protected backups per guest. Use '-1' for unlimited."]
    #[doc = ""]
    pub max_protected_backups: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: use 'prune-backups' instead. Maximal number of backup files per VM. Use '0' for unlimited."]
    #[doc = ""]
    pub maxfiles: Option<i64>,
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
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pool."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use this port to connect to the storage instead of the default one (for example, with PBS or ESXi). For NFS and CIFS, use the 'options' option to configure the port via the mount options."]
    #[doc = ""]
    pub port: Option<i64>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Wipe throughput (cstream -t parameter value)."]
    #[doc = ""]
    pub saferemove_throughput: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Server IP or DNS name."]
    #[doc = ""]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup volfile server IP or DNS name."]
    #[doc = ""]
    pub server2: Option<String>,
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
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use sparse volumes"]
    #[doc = ""]
    pub sparse: Option<bool>,
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
    #[doc = "Gluster transport: tcp or rdma"]
    #[doc = ""]
    pub transport: Option<Transport>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "RBD Id."]
    #[doc = ""]
    pub username: Option<String>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Preallocation mode for raw and qcow2 images. Using 'metadata' on raw images results in preallocation=off."]
#[doc = ""]
pub enum Preallocation {
    #[serde(rename = "falloc")]
    Falloc,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "metadata")]
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
impl Default for Preallocation {
    fn default() -> Self {
        Self::Metadata
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
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
impl Default for Smbversion {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Gluster transport: tcp or rdma"]
#[doc = ""]
pub enum Transport {
    #[serde(rename = "rdma")]
    Rdma,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "unix")]
    Unix,
}
impl TryFrom<&str> for Transport {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "rdma" => Ok(Self::Rdma),
            "tcp" => Ok(Self::Tcp),
            "unix" => Ok(Self::Unix),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The type of the created storage."]
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
    #[serde(rename = "glusterfs")]
    Glusterfs,
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
            "glusterfs" => Ok(Self::Glusterfs),
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
