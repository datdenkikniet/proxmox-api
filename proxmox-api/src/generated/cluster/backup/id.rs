pub mod included_volumes;
#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete vzdump backup job definition."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read vzdump backup job definition."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update vzdump backup job definition."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup all known guest systems on this host."]
    #[doc = ""]
    pub all: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit I/O bandwidth (in KiB/s)."]
    #[doc = ""]
    pub bwlimit: Option<BwlimitInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description for the Job."]
    #[doc = ""]
    pub comment: Option<CommentStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Compress dump file."]
    #[doc = ""]
    pub compress: Option<Compress>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Day of week selection."]
    #[doc = ""]
    pub dow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Store resulting files to specified directory."]
    #[doc = ""]
    pub dumpdir: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable or disable the job."]
    #[doc = ""]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Exclude specified guest systems (assumes --all)"]
    #[doc = ""]
    pub exclude: Option<String>,
    #[serde(rename = "exclude-path")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Exclude certain files/directories (shell globs). Paths starting with '/' are anchored to the container's root, other paths match relative to each subdirectory."]
    #[doc = ""]
    pub exclude_path: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Options for backup fleecing (VM only)."]
    #[doc = ""]
    pub fleecing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Set IO priority when using the BFQ scheduler. For snapshot and suspend mode backups of VMs, this only affects the compressor. A value of 8 means the idle priority is used, otherwise the best-effort priority is used with the specified value."]
    #[doc = ""]
    pub ionice: Option<IoniceInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal time to wait for the global lock (minutes)."]
    #[doc = ""]
    pub lockwait: Option<LockwaitInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: use notification targets/matchers instead. Specify when to send a notification mail"]
    #[doc = ""]
    pub mailnotification: Option<Mailnotification>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: Use notification targets/matchers instead. Comma-separated list of email addresses or users that should receive email notifications."]
    #[doc = ""]
    pub mailto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: use 'prune-backups' instead. Maximal number of backup files per guest system."]
    #[doc = ""]
    pub maxfiles: Option<MaxfilesInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup mode."]
    #[doc = ""]
    pub mode: Option<Mode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only run if executed on this node."]
    #[doc = ""]
    pub node: Option<String>,
    #[serde(rename = "notes-template")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Template string for generating notes for the backup(s). It can contain variables which will be replaced by their values. Currently supported are {{cluster}}, {{guestname}}, {{node}}, and {{vmid}}, but more might be added in the future. Needs to be a single line, newline and backslash need to be escaped as '\\n' and '\\\\' respectively."]
    #[doc = ""]
    pub notes_template: Option<NotesTemplateStr>,
    #[serde(rename = "notification-mode")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Determine which notification system to use. If set to 'legacy-sendmail', vzdump will consider the mailto/mailnotification parameters and send emails to the specified address(es) via the 'sendmail' command. If set to 'notification-system', a notification will be sent via PVE's notification system, and the mailto and mailnotification will be ignored. If set to 'auto' (default setting), an email will be sent if mailto is set, and the notification system will be used if not."]
    #[doc = ""]
    pub notification_mode: Option<NotificationMode>,
    #[serde(rename = "notification-policy")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: Do not use"]
    #[doc = ""]
    pub notification_policy: Option<NotificationPolicy>,
    #[serde(rename = "notification-target")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Deprecated: Do not use"]
    #[doc = ""]
    pub notification_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Other performance-related settings."]
    #[doc = ""]
    pub performance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use pigz instead of gzip when N\\\\>0. N=1 uses half of cores, N\\\\>1 uses N as thread count."]
    #[doc = ""]
    pub pigz: Option<PigzInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup all known guest systems included in the specified pool."]
    #[doc = ""]
    pub pool: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If true, mark backup(s) as protected."]
    #[doc = ""]
    pub protected: Option<bool>,
    #[serde(rename = "prune-backups")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use these retention options instead of those from the storage configuration."]
    #[doc = ""]
    pub prune_backups: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Be quiet."]
    #[doc = ""]
    pub quiet: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prune older backups according to 'prune-backups'."]
    #[doc = ""]
    pub remove: Option<bool>,
    #[serde(rename = "repeat-missed")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If true, the job will be run as soon as possible if it was missed while the scheduler was not running."]
    #[doc = ""]
    pub repeat_missed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Backup schedule. The format is a subset of `systemd` calendar events."]
    #[doc = ""]
    pub schedule: Option<ScheduleStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use specified hook script."]
    #[doc = ""]
    pub script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Job Start time."]
    #[doc = ""]
    pub starttime: Option<StarttimeStr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Exclude temporary files and logs."]
    #[doc = ""]
    pub stdexcludes: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Stop running backup jobs on this host."]
    #[doc = ""]
    pub stop: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximal time to wait until a guest system is stopped (minutes)."]
    #[doc = ""]
    pub stopwait: Option<StopwaitInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Store resulting file to this storage."]
    #[doc = ""]
    pub storage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Store temporary files to specified directory."]
    #[doc = ""]
    pub tmpdir: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The ID of the guest system you want to backup."]
    #[doc = ""]
    pub vmid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Zstd threads. N=0 uses half of the available cores, if N is set to a value bigger than 0, N is used as thread count."]
    #[doc = ""]
    pub zstd: Option<ZstdInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Compress dump file."]
#[doc = ""]
pub enum Compress {
    #[serde(rename = "0")]
    #[default]
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
impl TryFrom<&str> for Compress {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "0" => Ok(Self::_0),
            "1" => Ok(Self::_1),
            "gzip" => Ok(Self::Gzip),
            "lzo" => Ok(Self::Lzo),
            "zstd" => Ok(Self::Zstd),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Deprecated: use notification targets/matchers instead. Specify when to send a notification mail"]
#[doc = ""]
pub enum Mailnotification {
    #[serde(rename = "always")]
    #[default]
    Always,
    #[serde(rename = "failure")]
    Failure,
}
impl TryFrom<&str> for Mailnotification {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "always" => Ok(Self::Always),
            "failure" => Ok(Self::Failure),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Backup mode."]
#[doc = ""]
pub enum Mode {
    #[serde(rename = "snapshot")]
    #[default]
    Snapshot,
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "suspend")]
    Suspend,
}
impl TryFrom<&str> for Mode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "snapshot" => Ok(Self::Snapshot),
            "stop" => Ok(Self::Stop),
            "suspend" => Ok(Self::Suspend),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Determine which notification system to use. If set to 'legacy-sendmail', vzdump will consider the mailto/mailnotification parameters and send emails to the specified address(es) via the 'sendmail' command. If set to 'notification-system', a notification will be sent via PVE's notification system, and the mailto and mailnotification will be ignored. If set to 'auto' (default setting), an email will be sent if mailto is set, and the notification system will be used if not."]
#[doc = ""]
pub enum NotificationMode {
    #[serde(rename = "auto")]
    #[default]
    Auto,
    #[serde(rename = "legacy-sendmail")]
    LegacySendmail,
    #[serde(rename = "notification-system")]
    NotificationSystem,
}
impl TryFrom<&str> for NotificationMode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "auto" => Ok(Self::Auto),
            "legacy-sendmail" => Ok(Self::LegacySendmail),
            "notification-system" => Ok(Self::NotificationSystem),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Deprecated: Do not use"]
#[doc = ""]
pub enum NotificationPolicy {
    #[serde(rename = "always")]
    #[default]
    Always,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "never")]
    Never,
}
impl TryFrom<&str> for NotificationPolicy {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "always" => Ok(Self::Always),
            "failure" => Ok(Self::Failure),
            "never" => Ok(Self::Never),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BwlimitInt(i128);
impl crate::types::bounded_integer::BoundedInteger for BwlimitInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(0i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for BwlimitInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for BwlimitInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for BwlimitInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct IoniceInt(i128);
impl crate::types::bounded_integer::BoundedInteger for IoniceInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = Some(8i128);
    const DEFAULT: Option<i128> = Some(7i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 0 and 8";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for IoniceInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for IoniceInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for IoniceInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LockwaitInt(i128);
impl crate::types::bounded_integer::BoundedInteger for LockwaitInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(180i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for LockwaitInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for LockwaitInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for LockwaitInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MaxfilesInt(i128);
impl crate::types::bounded_integer::BoundedInteger for MaxfilesInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 1";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for MaxfilesInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for MaxfilesInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for MaxfilesInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PigzInt(i128);
impl crate::types::bounded_integer::BoundedInteger for PigzInt {
    const MIN: Option<i128> = None::<i128>;
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(0i128);
    const TYPE_DESCRIPTION: &'static str = "a valid integer";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for PigzInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for PigzInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PigzInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct StopwaitInt(i128);
impl crate::types::bounded_integer::BoundedInteger for StopwaitInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(10i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for StopwaitInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for StopwaitInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for StopwaitInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ZstdInt(i128);
impl crate::types::bounded_integer::BoundedInteger for ZstdInt {
    const MIN: Option<i128> = None::<i128>;
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(1i128);
    const TYPE_DESCRIPTION: &'static str = "a valid integer";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for ZstdInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for ZstdInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ZstdInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CommentStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for CommentStr {
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
impl std::convert::TryFrom<String> for CommentStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for CommentStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CommentStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NotesTemplateStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for NotesTemplateStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(1024usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 1024";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for NotesTemplateStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for NotesTemplateStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NotesTemplateStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ScheduleStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for ScheduleStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(128usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 128";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for ScheduleStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for ScheduleStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ScheduleStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct StarttimeStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for StarttimeStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("\\d{1,2}:\\d{1,2}");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"\\d{1,2}:\\d{1,2}\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for StarttimeStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for StarttimeStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for StarttimeStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    pub fn included_volumes(&self) -> included_volumes::IncludedVolumesClient<T> {
        included_volumes::IncludedVolumesClient::<T>::new(self.client.clone(), &self.path)
    }
}
