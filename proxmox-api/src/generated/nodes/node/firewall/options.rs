#[derive(Debug, Clone)]
pub struct OptionsClient<T> {
    client: T,
    path: String,
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/options"),
        }
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get host firewall options."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set Firewall options."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable host firewall rules."]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for incoming traffic."]
    #[doc = ""]
    pub log_level_in: Option<LogLevelIn>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for outgoing traffic."]
    #[doc = ""]
    pub log_level_out: Option<LogLevelOut>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable logging of conntrack information."]
    #[doc = ""]
    pub log_nf_conntrack: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable NDP (Neighbor Discovery Protocol)."]
    #[doc = ""]
    pub ndp: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow invalid packets on connection tracking."]
    #[doc = ""]
    pub nf_conntrack_allow_invalid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable conntrack helpers for specific protocols. Supported protocols: amanda, ftp, irc, netbios-ns, pptp, sane, sip, snmp, tftp"]
    #[doc = ""]
    pub nf_conntrack_helpers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum number of tracked connections."]
    #[doc = ""]
    pub nf_conntrack_max: Option<NfConntrackMaxInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Conntrack established timeout."]
    #[doc = ""]
    pub nf_conntrack_tcp_timeout_established: Option<NfConntrackTcpTimeoutEstablishedInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Conntrack syn recv timeout."]
    #[doc = ""]
    pub nf_conntrack_tcp_timeout_syn_recv: Option<NfConntrackTcpTimeoutSynRecvInt>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable nftables based firewall (tech preview)"]
    #[doc = ""]
    pub nftables: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable SMURFS filter."]
    #[doc = ""]
    pub nosmurfs: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable synflood protection"]
    #[doc = ""]
    pub protection_synflood: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Synflood protection rate burst by ip src."]
    #[doc = ""]
    pub protection_synflood_burst: Option<ProtectionSynfloodBurstInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Synflood protection rate syn/sec by ip src."]
    #[doc = ""]
    pub protection_synflood_rate: Option<ProtectionSynfloodRateInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for SMURFS filter."]
    #[doc = ""]
    pub smurf_log_level: Option<SmurfLogLevel>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for illegal tcp flags filter."]
    #[doc = ""]
    pub tcp_flags_log_level: Option<TcpFlagsLogLevel>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Filter illegal combinations of TCP flags."]
    #[doc = ""]
    pub tcpflags: Option<bool>,
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
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable host firewall rules."]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for incoming traffic."]
    #[doc = ""]
    pub log_level_in: Option<LogLevelIn>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for outgoing traffic."]
    #[doc = ""]
    pub log_level_out: Option<LogLevelOut>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable logging of conntrack information."]
    #[doc = ""]
    pub log_nf_conntrack: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable NDP (Neighbor Discovery Protocol)."]
    #[doc = ""]
    pub ndp: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow invalid packets on connection tracking."]
    #[doc = ""]
    pub nf_conntrack_allow_invalid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable conntrack helpers for specific protocols. Supported protocols: amanda, ftp, irc, netbios-ns, pptp, sane, sip, snmp, tftp"]
    #[doc = ""]
    pub nf_conntrack_helpers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Maximum number of tracked connections."]
    #[doc = ""]
    pub nf_conntrack_max: Option<NfConntrackMaxInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Conntrack established timeout."]
    #[doc = ""]
    pub nf_conntrack_tcp_timeout_established: Option<NfConntrackTcpTimeoutEstablishedInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Conntrack syn recv timeout."]
    #[doc = ""]
    pub nf_conntrack_tcp_timeout_syn_recv: Option<NfConntrackTcpTimeoutSynRecvInt>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable nftables based firewall (tech preview)"]
    #[doc = ""]
    pub nftables: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable SMURFS filter."]
    #[doc = ""]
    pub nosmurfs: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable synflood protection"]
    #[doc = ""]
    pub protection_synflood: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Synflood protection rate burst by ip src."]
    #[doc = ""]
    pub protection_synflood_burst: Option<ProtectionSynfloodBurstInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Synflood protection rate syn/sec by ip src."]
    #[doc = ""]
    pub protection_synflood_rate: Option<ProtectionSynfloodRateInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for SMURFS filter."]
    #[doc = ""]
    pub smurf_log_level: Option<SmurfLogLevel>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for illegal tcp flags filter."]
    #[doc = ""]
    pub tcp_flags_log_level: Option<TcpFlagsLogLevel>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Filter illegal combinations of TCP flags."]
    #[doc = ""]
    pub tcpflags: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Log level for incoming traffic."]
#[doc = ""]
pub enum LogLevelIn {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "crit")]
    Crit,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emerg")]
    Emerg,
    #[serde(rename = "err")]
    Err,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "nolog")]
    Nolog,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
impl TryFrom<&str> for LogLevelIn {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "alert" => Ok(Self::Alert),
            "crit" => Ok(Self::Crit),
            "debug" => Ok(Self::Debug),
            "emerg" => Ok(Self::Emerg),
            "err" => Ok(Self::Err),
            "info" => Ok(Self::Info),
            "nolog" => Ok(Self::Nolog),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Log level for outgoing traffic."]
#[doc = ""]
pub enum LogLevelOut {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "crit")]
    Crit,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emerg")]
    Emerg,
    #[serde(rename = "err")]
    Err,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "nolog")]
    Nolog,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
impl TryFrom<&str> for LogLevelOut {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "alert" => Ok(Self::Alert),
            "crit" => Ok(Self::Crit),
            "debug" => Ok(Self::Debug),
            "emerg" => Ok(Self::Emerg),
            "err" => Ok(Self::Err),
            "info" => Ok(Self::Info),
            "nolog" => Ok(Self::Nolog),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Log level for SMURFS filter."]
#[doc = ""]
pub enum SmurfLogLevel {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "crit")]
    Crit,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emerg")]
    Emerg,
    #[serde(rename = "err")]
    Err,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "nolog")]
    Nolog,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
impl TryFrom<&str> for SmurfLogLevel {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "alert" => Ok(Self::Alert),
            "crit" => Ok(Self::Crit),
            "debug" => Ok(Self::Debug),
            "emerg" => Ok(Self::Emerg),
            "err" => Ok(Self::Err),
            "info" => Ok(Self::Info),
            "nolog" => Ok(Self::Nolog),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Log level for illegal tcp flags filter."]
#[doc = ""]
pub enum TcpFlagsLogLevel {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "crit")]
    Crit,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emerg")]
    Emerg,
    #[serde(rename = "err")]
    Err,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "nolog")]
    Nolog,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
impl TryFrom<&str> for TcpFlagsLogLevel {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "alert" => Ok(Self::Alert),
            "crit" => Ok(Self::Crit),
            "debug" => Ok(Self::Debug),
            "emerg" => Ok(Self::Emerg),
            "err" => Ok(Self::Err),
            "info" => Ok(Self::Info),
            "nolog" => Ok(Self::Nolog),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NfConntrackMaxInt(i128);
impl crate::types::bounded_integer::BoundedInteger for NfConntrackMaxInt {
    const MIN: Option<i128> = Some(32768i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(262144i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 32768";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for NfConntrackMaxInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for NfConntrackMaxInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NfConntrackMaxInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NfConntrackTcpTimeoutEstablishedInt(i128);
impl crate::types::bounded_integer::BoundedInteger for NfConntrackTcpTimeoutEstablishedInt {
    const MIN: Option<i128> = Some(7875i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(432000i128);
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 7875";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for NfConntrackTcpTimeoutEstablishedInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for NfConntrackTcpTimeoutEstablishedInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NfConntrackTcpTimeoutEstablishedInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NfConntrackTcpTimeoutSynRecvInt(i128);
impl crate::types::bounded_integer::BoundedInteger for NfConntrackTcpTimeoutSynRecvInt {
    const MIN: Option<i128> = Some(30i128);
    const MAX: Option<i128> = Some(60i128);
    const DEFAULT: Option<i128> = Some(60i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 30 and 60";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for NfConntrackTcpTimeoutSynRecvInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for NfConntrackTcpTimeoutSynRecvInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for NfConntrackTcpTimeoutSynRecvInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ProtectionSynfloodBurstInt(i128);
impl crate::types::bounded_integer::BoundedInteger for ProtectionSynfloodBurstInt {
    const MIN: Option<i128> = None::<i128>;
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(1000i128);
    const TYPE_DESCRIPTION: &'static str = "a valid integer";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for ProtectionSynfloodBurstInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for ProtectionSynfloodBurstInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ProtectionSynfloodBurstInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ProtectionSynfloodRateInt(i128);
impl crate::types::bounded_integer::BoundedInteger for ProtectionSynfloodRateInt {
    const MIN: Option<i128> = None::<i128>;
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = Some(200i128);
    const TYPE_DESCRIPTION: &'static str = "a valid integer";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for ProtectionSynfloodRateInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for ProtectionSynfloodRateInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ProtectionSynfloodRateInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DigestStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DigestStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(64usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 64";
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
