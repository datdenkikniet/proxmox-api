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
    #[doc = "Get VM firewall options."]
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
    #[doc = "Enable DHCP."]
    #[doc = ""]
    pub dhcp: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable firewall rules."]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable default IP filters. This is equivalent to adding an empty ipfilter-net\\\\<id\\\\> ipset for every interface. Such ipsets implicitly contain sane default restrictions such as restricting IPv6 link local addresses to the one derived from the interface's MAC address. For containers the configured IP addresses will be implicitly added."]
    #[doc = ""]
    pub ipfilter: Option<bool>,
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
    #[doc = "Enable/disable MAC address filter."]
    #[doc = ""]
    pub macfilter: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable NDP (Neighbor Discovery Protocol)."]
    #[doc = ""]
    pub ndp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Input policy."]
    #[doc = ""]
    pub policy_in: Option<PolicyIn>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Output policy."]
    #[doc = ""]
    pub policy_out: Option<PolicyOut>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow sending Router Advertisement."]
    #[doc = ""]
    pub radv: Option<bool>,
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
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable DHCP."]
    #[doc = ""]
    pub dhcp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable firewall rules."]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable default IP filters. This is equivalent to adding an empty ipfilter-net\\\\<id\\\\> ipset for every interface. Such ipsets implicitly contain sane default restrictions such as restricting IPv6 link local addresses to the one derived from the interface's MAC address. For containers the configured IP addresses will be implicitly added."]
    #[doc = ""]
    pub ipfilter: Option<bool>,
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
    #[doc = "Enable/disable MAC address filter."]
    #[doc = ""]
    pub macfilter: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable NDP (Neighbor Discovery Protocol)."]
    #[doc = ""]
    pub ndp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Input policy."]
    #[doc = ""]
    pub policy_in: Option<PolicyIn>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Output policy."]
    #[doc = ""]
    pub policy_out: Option<PolicyOut>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow sending Router Advertisement."]
    #[doc = ""]
    pub radv: Option<bool>,
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
#[doc = "Input policy."]
#[doc = ""]
pub enum PolicyIn {
    ACCEPT,
    DROP,
    REJECT,
}
impl TryFrom<&str> for PolicyIn {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ACCEPT" => Ok(Self::ACCEPT),
            "DROP" => Ok(Self::DROP),
            "REJECT" => Ok(Self::REJECT),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Output policy."]
#[doc = ""]
pub enum PolicyOut {
    ACCEPT,
    DROP,
    REJECT,
}
impl TryFrom<&str> for PolicyOut {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ACCEPT" => Ok(Self::ACCEPT),
            "DROP" => Ok(Self::DROP),
            "REJECT" => Ok(Self::REJECT),
            v => Err(format!("Unknown variant {v}")),
        }
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
