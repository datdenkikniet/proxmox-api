#[derive(Debug, Clone)]
pub struct ZoneClient<T> {
    client: T,
    path: String,
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, zone: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, zone),
        }
    }
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete sdn zone object configuration."]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read sdn zone configuration."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update sdn zone object configuration."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(rename = "lock-token")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the token for unlocking the global SDN configuration"]
    #[doc = ""]
    pub lock_token: Option<String>,
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
    #[doc = "Display pending config."]
    #[doc = ""]
    pub pending: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display running config."]
    #[doc = ""]
    pub running: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(rename = "advertise-subnets")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Advertise IP prefixes (Type-5 routes) instead of MAC/IP pairs (Type-2 routes)."]
    #[doc = ""]
    pub advertise_subnets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The bridge for which VLANs should be managed."]
    #[doc = ""]
    pub bridge: Option<String>,
    #[serde(rename = "bridge-disable-mac-learning")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable auto mac learning."]
    #[doc = ""]
    pub bridge_disable_mac_learning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Controller for this zone."]
    #[doc = ""]
    pub controller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<DeleteStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Type of the DHCP backend for this zone"]
    #[doc = ""]
    pub dhcp: Option<Dhcp>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(rename = "disable-arp-nd-suppression")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Suppress IPv4 ARP && IPv6 Neighbour Discovery messages."]
    #[doc = ""]
    pub disable_arp_nd_suppression: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "dns api server"]
    #[doc = ""]
    pub dns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "dns domain zone  ex: mydomain.com"]
    #[doc = ""]
    pub dnszone: Option<String>,
    #[serde(rename = "dp-id")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Faucet dataplane id"]
    #[doc = ""]
    pub dp_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of cluster node names."]
    #[doc = ""]
    pub exitnodes: Option<String>,
    #[serde(rename = "exitnodes-local-routing")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow exitnodes to connect to EVPN guests."]
    #[doc = ""]
    pub exitnodes_local_routing: Option<bool>,
    #[serde(rename = "exitnodes-primary")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force traffic through this exitnode first."]
    #[doc = ""]
    pub exitnodes_primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "SDN fabric to use as underlay for this VXLAN zone."]
    #[doc = ""]
    pub fabric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use a specific ipam"]
    #[doc = ""]
    pub ipam: Option<String>,
    #[serde(rename = "lock-token")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the token for unlocking the global SDN configuration"]
    #[doc = ""]
    pub lock_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Anycast logical router mac address."]
    #[doc = ""]
    pub mac: Option<crate::types::MacAddr<true>>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU of the zone, will be used for the created VNet bridges."]
    #[doc = ""]
    pub mtu: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of cluster node names."]
    #[doc = ""]
    pub nodes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comma-separated list of peers, that are part of the VXLAN zone. Usually the IPs of the nodes."]
    #[doc = ""]
    pub peers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "reverse dns api server"]
    #[doc = ""]
    pub reversedns: Option<String>,
    #[serde(rename = "rt-import")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of Route Targets that should be imported into the VRF of the zone."]
    #[doc = ""]
    pub rt_import: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Service-VLAN Tag (outer VLAN)"]
    #[doc = ""]
    pub tag: Option<u64>,
    #[serde(rename = "vlan-protocol")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Which VLAN protocol should be used for the creation of the QinQ zone."]
    #[doc = ""]
    pub vlan_protocol: Option<VlanProtocol>,
    #[serde(rename = "vrf-vxlan")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VNI for the zone VRF."]
    #[doc = ""]
    pub vrf_vxlan: Option<VrfVxlanInt>,
    #[serde(rename = "vxlan-port")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "UDP port that should be used for the VXLAN tunnel (default 4789)."]
    #[doc = ""]
    pub vxlan_port: Option<VxlanPortInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Type of the DHCP backend for this zone"]
#[doc = ""]
pub enum Dhcp {
    #[serde(rename = "dnsmasq")]
    Dnsmasq,
}
impl TryFrom<&str> for Dhcp {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "dnsmasq" => Ok(Self::Dnsmasq),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "Which VLAN protocol should be used for the creation of the QinQ zone."]
#[doc = ""]
pub enum VlanProtocol {
    #[serde(rename = "802.1ad")]
    _8021ad,
    #[serde(rename = "802.1q")]
    #[default]
    _8021q,
}
impl TryFrom<&str> for VlanProtocol {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "802.1ad" => Ok(Self::_8021ad),
            "802.1q" => Ok(Self::_8021q),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VrfVxlanInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VrfVxlanInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(16777215i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 16777215";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VrfVxlanInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VrfVxlanInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VrfVxlanInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VxlanPortInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VxlanPortInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(65536i128);
    const DEFAULT: Option<i128> = Some(4789i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 65536";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VxlanPortInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VxlanPortInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VxlanPortInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DeleteStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DeleteStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(4096usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 4096";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DeleteStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DeleteStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DeleteStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
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
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DigestStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
