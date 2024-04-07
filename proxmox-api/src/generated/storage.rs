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
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Storage index."]
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
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct ConfigPostOutputConfig {
    #[serde(rename = "encryption-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The, possible auto-generated, encryption-key."]
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
    pub config: Option<ConfigPostOutputConfig>,
    #[doc = "The ID of the created storage."]
    pub storage: String,
    #[serde(rename = "type")]
    #[doc = "The type of the created storage."]
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
    pub authsupported: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Base volume. This volume is automatically activated."]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "block size"]
    pub blocksize: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set I/O bandwidth limit for various operations (in KiB/s)."]
    pub bwlimit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "host group for comstar views"]
    pub comstar_hg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "target group for comstar views"]
    pub comstar_tg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allowed content types.\n\nNOTE: the value 'rootdir' is used for Containers, and value 'images' for VMs.\n"]
    pub content: Option<String>,
    #[serde(rename = "content-dirs")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Overrides for default content type directories."]
    pub content_dirs: Option<String>,
    #[serde(rename = "create-base-path")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create the base directory if it doesn't exist."]
    pub create_base_path: Option<()>,
    #[serde(rename = "create-subdirs")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Populate the directory with the default structure."]
    pub create_subdirs: Option<()>,
    #[serde(rename = "data-pool")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Data Pool (for erasure coding only)"]
    pub data_pool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Proxmox Backup Server datastore name."]
    pub datastore: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Flag to disable the storage."]
    pub disable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CIFS domain."]
    pub domain: Option<String>,
    #[serde(rename = "encryption-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Encryption key. Use 'autogen' to generate one automatically without passphrase."]
    pub encryption_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "NFS export path."]
    pub export: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default image format."]
    pub format: Option<String>,
    #[serde(rename = "fs-name")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The Ceph filesystem name."]
    pub fs_name: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mount CephFS through FUSE."]
    pub fuse: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Assume the given path is an externally managed mountpoint and consider the storage offline if it is not mounted. Using a boolean (yes/no) value serves as a shortcut to using the target path in this field."]
    pub is_mountpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "iscsi provider"]
    pub iscsiprovider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Client keyring contents (for external clusters)."]
    pub keyring: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Always access rbd through krbd kernel module."]
    pub krbd: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "target portal group for Linux LIO targets"]
    pub lio_tpg: Option<String>,
    #[serde(rename = "master-pubkey")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Base64-encoded, PEM-formatted public RSA key. Used to encrypt a copy of the encryption-key which will be added to each encrypted backup."]
    pub master_pubkey: Option<String>,
    #[serde(rename = "max-protected-backups")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal number of protected backups per guest. Use '-1' for unlimited."]
    pub max_protected_backups: Option<()>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: use 'prune-backups' instead. Maximal number of backup files per VM. Use '0' for unlimited."]
    pub maxfiles: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create the directory if it doesn't exist and populate it with default sub-dirs. NOTE: Deprecated, use the 'create-base-path' and 'create-subdirs' options instead."]
    pub mkdir: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP addresses of monitors (for external clusters)."]
    pub monhost: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "mount point"]
    pub mountpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Namespace."]
    pub namespace: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set the NOCOW flag on files. Disables data checksumming and causes data errors to be unrecoverable from while allowing direct I/O. Only use this if data does not need to be any more safe than on a single ext4 formatted disk with no underlying raid system."]
    pub nocow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of nodes for which the storage configuration applies."]
    pub nodes: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "disable write caching on the target"]
    pub nowritecache: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "NFS/CIFS mount options (see 'man nfs' or 'man mount.cifs')"]
    pub options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Password for accessing the share/datastore."]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "File system path."]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Pool."]
    pub pool: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For non default port."]
    pub port: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "iSCSI portal (IP or DNS name with optional port)."]
    pub portal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Preallocation mode for raw and qcow2 images. Using 'metadata' on raw images results in preallocation=off."]
    pub preallocation: Option<Preallocation>,
    #[serde(rename = "prune-backups")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The retention options with shorter intervals are processed first with --keep-last being the very first one. Each option covers a specific period of time. We say that backups within this period are covered by this option. The next option does not take care of already covered backups and only considers older backups."]
    pub prune_backups: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Zero-out data when removing LVs."]
    pub saferemove: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Wipe throughput (cstream -t parameter value)."]
    pub saferemove_throughput: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Server IP or DNS name."]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup volfile server IP or DNS name."]
    pub server2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CIFS share."]
    pub share: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Indicate that this is a single storage with the same contents on all nodes (or all listed in the 'nodes' option). It will not make the contents of a local storage automatically accessible to other nodes, it just marks an already shared storage as such!"]
    pub shared: Option<bool>,
    #[serde(rename = "skip-cert-verification")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable TLS certificate verification, only enable on fully trusted networks!"]
    pub skip_cert_verification: Option<()>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "SMB protocol version. 'default' if not set, negotiates the highest SMB2+ version supported by both the client and server."]
    pub smbversion: Option<Smbversion>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use sparse volumes"]
    pub sparse: Option<bool>,
    #[doc = "The storage identifier."]
    pub storage: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Subdir to mount."]
    pub subdir: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only use logical volumes tagged with 'pve-vm-ID'."]
    pub tagged_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "iSCSI target."]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "LVM thin pool LV name."]
    pub thinpool: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Gluster transport: tcp or rdma"]
    pub transport: Option<Transport>,
    #[serde(rename = "type")]
    #[doc = "Storage type."]
    pub ty: Type,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "RBD Id."]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Volume group name."]
    pub vgname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Glusterfs Volume."]
    pub volume: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
impl Default for Preallocation {
    fn default() -> Self {
        Self::Metadata
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
impl Default for Smbversion {
    fn default() -> Self {
        Self::Default
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Transport {
    #[serde(rename = "rdma")]
    Rdma,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "unix")]
    Unix,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn storage(&self, storage: &str) -> storage::StorageClient<T> {
        storage::StorageClient::<T>::new(self.client.clone(), &self.path, storage)
    }
}
