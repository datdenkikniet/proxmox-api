#[derive(Debug, Clone)]
pub struct PosClient<T> {
    client: T,
    path: String,
}
impl<T> PosClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, pos: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, pos),
        }
    }
}
impl<T> PosClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete rule."]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl<T> PosClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get single rule data."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> PosClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Modify rule data."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(action: String, pos: u64, ty: String) -> Self {
        Self {
            action,
            pos,
            ty,
            comment: Default::default(),
            dest: Default::default(),
            dport: Default::default(),
            enable: Default::default(),
            icmp_type: Default::default(),
            iface: Default::default(),
            ipversion: Default::default(),
            log: Default::default(),
            macro_def: Default::default(),
            proto: Default::default(),
            source: Default::default(),
            sport: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub dport: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub enable: Option<u64>,
    #[serde(rename = "icmp-type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub icmp_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub iface: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ipversion: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for firewall rule"]
    #[doc = ""]
    pub log: Option<Log>,
    #[serde(rename = "macro")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub macro_def: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pos: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub proto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sport: Option<String>,
    #[serde(rename = "type")]
    pub ty: String,
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
    #[doc = "Rule action ('ACCEPT', 'DROP', 'REJECT') or security group name."]
    #[doc = ""]
    pub action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Descriptive comment."]
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Restrict packet destination address. This can refer to a single IP address, an IP set ('+ipsetname') or an IP alias definition. You can also specify an address range like '20.34.101.207-201.3.9.99', or a list of IP addresses and networks (entries are separated by comma). Please do not mix IPv4 and IPv6 addresses inside such lists."]
    #[doc = ""]
    pub dest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Restrict TCP/UDP destination port. You can use service names or simple numbers (0-65535), as defined in '/etc/services'. Port ranges can be specified with '\\d+:\\d+', for example '80:85', and you can use comma separated list to match several ports or ranges."]
    #[doc = ""]
    pub dport: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Flag to enable/disable a rule."]
    #[doc = ""]
    pub enable: Option<u64>,
    #[serde(rename = "icmp-type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify icmp-type. Only valid if proto equals 'icmp' or 'icmpv6'/'ipv6-icmp'."]
    #[doc = ""]
    pub icmp_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Network interface name. You have to use network configuration key names for VMs and containers ('net\\d+'). Host related rules can use arbitrary strings."]
    #[doc = ""]
    pub iface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for firewall rule."]
    #[doc = ""]
    pub log: Option<Log>,
    #[serde(rename = "macro")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use predefined standard macro."]
    #[doc = ""]
    pub macro_def: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Move rule to new position \\\\<moveto\\\\>. Other arguments are ignored."]
    #[doc = ""]
    pub moveto: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP protocol. You can use protocol names ('tcp'/'udp') or simple numbers, as defined in '/etc/protocols'."]
    #[doc = ""]
    pub proto: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Restrict packet source address. This can refer to a single IP address, an IP set ('+ipsetname') or an IP alias definition. You can also specify an address range like '20.34.101.207-201.3.9.99', or a list of IP addresses and networks (entries are separated by comma). Please do not mix IPv4 and IPv6 addresses inside such lists."]
    #[doc = ""]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Restrict TCP/UDP source port. You can use service names or simple numbers (0-65535), as defined in '/etc/services'. Port ranges can be specified with '\\d+:\\d+', for example '80:85', and you can use comma separated list to match several ports or ranges."]
    #[doc = ""]
    pub sport: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Rule type."]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Log level for firewall rule"]
#[doc = ""]
pub enum Log {
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
impl TryFrom<&str> for Log {
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
#[doc = "Rule type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "out")]
    Out,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "group" => Ok(Self::Group),
            "in" => Ok(Self::In),
            "out" => Ok(Self::Out),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
