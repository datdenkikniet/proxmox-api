pub mod defaults;
pub mod extractconfig;
pub struct VzdumpClient<T> {
    client: T,
    path: String,
}
impl<T> VzdumpClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/vzdump"),
        }
    }
}
impl<T> VzdumpClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create backup."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup all known guest systems on this host."]
    pub all: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit I/O bandwidth (in KiB/s)."]
    pub bwlimit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Compress dump file."]
    pub compress: Option<Compress>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Store resulting files to specified directory."]
    pub dumpdir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Exclude specified guest systems (assumes --all)"]
    pub exclude: Option<String>,
    #[serde(rename = "exclude-path")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Exclude certain files/directories (shell globs). Paths starting with '/' are anchored to the container's root, other paths match relative to each subdirectory."]
    pub exclude_path: Vec<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set IO priority when using the BFQ scheduler. For snapshot and suspend mode backups of VMs, this only affects the compressor. A value of 8 means the idle priority is used, otherwise the best-effort priority is used with the specified value."]
    pub ionice: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal time to wait for the global lock (minutes)."]
    pub lockwait: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: use notification targets/matchers instead. Specify when to send a notification mail"]
    pub mailnotification: Option<Mailnotification>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: Use notification targets/matchers instead. Comma-separated list of email addresses or users that should receive email notifications."]
    pub mailto: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: use 'prune-backups' instead. Maximal number of backup files per guest system."]
    pub maxfiles: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup mode."]
    pub mode: Option<Mode>,
    #[serde(rename = "notes-template")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Template string for generating notes for the backup(s). It can contain variables which will be replaced by their values. Currently supported are {{cluster}}, {{guestname}}, {{node}}, and {{vmid}}, but more might be added in the future. Needs to be a single line, newline and backslash need to be escaped as '\\n' and '\\\\' respectively."]
    pub notes_template: Option<String>,
    #[serde(rename = "notification-mode")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determine which notification system to use. If set to 'legacy-sendmail', vzdump will consider the mailto/mailnotification parameters and send emails to the specified address(es) via the 'sendmail' command. If set to 'notification-system', a notification will be sent via PVE's notification system, and the mailto and mailnotification will be ignored. If set to 'auto' (default setting), an email will be sent if mailto is set, and the notification system will be used if not."]
    pub notification_mode: Option<NotificationMode>,
    #[serde(rename = "notification-policy")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: Do not use"]
    pub notification_policy: Option<NotificationPolicy>,
    #[serde(rename = "notification-target")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: Do not use"]
    pub notification_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Other performance-related settings."]
    pub performance: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use pigz instead of gzip when N\\>0. N=1 uses half of cores, N\\>1 uses N as thread count."]
    pub pigz: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup all known guest systems included in the specified pool."]
    pub pool: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If true, mark backup(s) as protected."]
    pub protected: Option<bool>,
    #[serde(rename = "prune-backups")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use these retention options instead of those from the storage configuration."]
    pub prune_backups: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Be quiet."]
    pub quiet: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prune older backups according to 'prune-backups'."]
    pub remove: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use specified hook script."]
    pub script: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Exclude temporary files and logs."]
    pub stdexcludes: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Write tar to stdout, not to a file."]
    pub stdout: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Stop running backup jobs on this host."]
    pub stop: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal time to wait until a guest system is stopped (minutes)."]
    pub stopwait: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Store resulting file to this storage."]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Store temporary files to specified directory."]
    pub tmpdir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The ID of the guest system you want to backup."]
    pub vmid: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Zstd threads. N=0 uses half of the available cores, if N is set to a value bigger than 0, N is used as thread count."]
    pub zstd: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Compress {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "lzo")]
    Lzo,
    #[serde(rename = "zstd")]
    Zstd,
}
impl Default for Compress {
    fn default() -> Self {
        Self::_0
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Mailnotification {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "failure")]
    Failure,
}
impl Default for Mailnotification {
    fn default() -> Self {
        Self::Always
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Mode {
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "suspend")]
    Suspend,
}
impl Default for Mode {
    fn default() -> Self {
        Self::Snapshot
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum NotificationMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "legacy-sendmail")]
    LegacySendmail,
    #[serde(rename = "notification-system")]
    NotificationSystem,
}
impl Default for NotificationMode {
    fn default() -> Self {
        Self::Auto
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum NotificationPolicy {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "never")]
    Never,
}
impl Default for NotificationPolicy {
    fn default() -> Self {
        Self::Always
    }
}
impl<T> VzdumpClient<T>
where
    T: crate::client::Client,
{
    pub fn defaults(&self) -> defaults::DefaultsClient<T> {
        defaults::DefaultsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VzdumpClient<T>
where
    T: crate::client::Client,
{
    pub fn extractconfig(&self) -> extractconfig::ExtractconfigClient<T> {
        extractconfig::ExtractconfigClient::<T>::new(self.client.clone(), &self.path)
    }
}
