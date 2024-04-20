pub mod storage;
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
impl<'a, T> crate::ProxmoxClient for &'a StorageClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Storage index."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<GetParams, Vec<GetOutputItems>, T::Error>
    for &StorageClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get(params)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new storage."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, PostOutput, T::Error>
    for &StorageClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        self.post(params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct ConfigPostOutputConfig {
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
impl GetOutputItems {
    pub fn new(storage: String) -> Self {
        Self {
            storage,
            additional_properties: Default::default(),
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
            config: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Partial, possible server generated, configuration properties."]
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
            authsupported: Default::default(),
            base: Default::default(),
            blocksize: Default::default(),
            bwlimit: Default::default(),
            comstar_hg: Default::default(),
            comstar_tg: Default::default(),
            content: Default::default(),
            content_dirs: Default::default(),
            create_base_path: Default::default(),
            create_subdirs: Default::default(),
            data_pool: Default::default(),
            datastore: Default::default(),
            disable: Default::default(),
            domain: Default::default(),
            encryption_key: Default::default(),
            export: Default::default(),
            fingerprint: Default::default(),
            format: Default::default(),
            fs_name: Default::default(),
            fuse: Default::default(),
            is_mountpoint: Default::default(),
            iscsiprovider: Default::default(),
            keyring: Default::default(),
            krbd: Default::default(),
            lio_tpg: Default::default(),
            master_pubkey: Default::default(),
            max_protected_backups: Default::default(),
            maxfiles: Default::default(),
            mkdir: Default::default(),
            monhost: Default::default(),
            mountpoint: Default::default(),
            namespace: Default::default(),
            nocow: Default::default(),
            nodes: Default::default(),
            nowritecache: Default::default(),
            options: Default::default(),
            password: Default::default(),
            path: Default::default(),
            pool: Default::default(),
            port: Default::default(),
            portal: Default::default(),
            preallocation: Default::default(),
            prune_backups: Default::default(),
            saferemove: Default::default(),
            saferemove_throughput: Default::default(),
            server: Default::default(),
            server2: Default::default(),
            share: Default::default(),
            shared: Default::default(),
            skip_cert_verification: Default::default(),
            smbversion: Default::default(),
            sparse: Default::default(),
            subdir: Default::default(),
            tagged_only: Default::default(),
            target: Default::default(),
            thinpool: Default::default(),
            transport: Default::default(),
            username: Default::default(),
            vgname: Default::default(),
            volume: Default::default(),
            additional_properties: Default::default(),
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
    pub domain: Option<String>,
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
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default image format."]
    #[doc = ""]
    pub format: Option<String>,
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
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of protected backups per guest. Use '-1' for unlimited."]
    #[doc = ""]
    pub max_protected_backups: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: use 'prune-backups' instead. Maximal number of backup files per VM. Use '0' for unlimited."]
    #[doc = ""]
    pub maxfiles: Option<u64>,
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
    #[doc = "File system path."]
    #[doc = ""]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pool."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For non default port."]
    #[doc = ""]
    pub port: Option<u64>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Gluster transport: tcp or rdma"]
    #[doc = ""]
    pub transport: Option<Transport>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Glusterfs Volume."]
    #[doc = ""]
    pub volume: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn storage(&self, storage: &str) -> storage::StorageClient<T> {
        storage::StorageClient::<T>::new(self.client.clone(), &self.path, storage)
    }
}
