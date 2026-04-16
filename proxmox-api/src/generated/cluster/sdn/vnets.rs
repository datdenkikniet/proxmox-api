pub mod vnet;
#[derive(Debug, Clone)]
pub struct VnetsClient<T> {
    client: T,
    path: String,
}
impl<T> VnetsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/vnets"),
        }
    }
}
impl<T> VnetsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN vnets index."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> VnetsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new sdn vnet object."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(ty: Type, vnet: String) -> Self {
        Self {
            ty,
            vnet,
            alias: ::std::default::Default::default(),
            digest: ::std::default::Default::default(),
            isolate_ports: ::std::default::Default::default(),
            pending: ::std::default::Default::default(),
            state: ::std::default::Default::default(),
            tag: ::std::default::Default::default(),
            vlanaware: ::std::default::Default::default(),
            zone: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Alias name of the VNet."]
    #[doc = ""]
    pub alias: Option<AliasStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Digest of the VNet section."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(rename = "isolate-ports")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If true, sets the isolated property for all interfaces on the bridge of this VNet."]
    #[doc = ""]
    pub isolate_ports: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Changes that have not yet been applied to the running configuration."]
    #[doc = ""]
    pub pending: Option<PendingGetOutputItemsPending>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "State of the SDN configuration object."]
    #[doc = ""]
    pub state: Option<State>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VLAN Tag (for VLAN or QinQ zones) or VXLAN VNI (for VXLAN or EVPN zones)."]
    #[doc = ""]
    pub tag: Option<TagInt>,
    #[serde(rename = "type")]
    #[doc = "Type of the VNet."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow VLANs to pass through this VNet."]
    #[doc = ""]
    pub vlanaware: Option<bool>,
    #[doc = "Name of the VNet."]
    #[doc = ""]
    pub vnet: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of the zone this VNet belongs to."]
    #[doc = ""]
    pub zone: Option<String>,
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
pub struct PendingGetOutputItemsPending {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Alias name of the VNet."]
    #[doc = ""]
    pub alias: Option<AliasStr>,
    #[serde(rename = "isolate-ports")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If true, sets the isolated property for all interfaces on the bridge of this VNet."]
    #[doc = ""]
    pub isolate_ports: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VLAN Tag (for VLAN or QinQ zones) or VXLAN VNI (for VXLAN or EVPN zones)."]
    #[doc = ""]
    pub tag: Option<TagInt>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow VLANs to pass through this VNet."]
    #[doc = ""]
    pub vlanaware: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of the zone this VNet belongs to."]
    #[doc = ""]
    pub zone: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(vnet: String, zone: String) -> Self {
        Self {
            vnet,
            zone,
            alias: ::std::default::Default::default(),
            isolate_ports: ::std::default::Default::default(),
            lock_token: ::std::default::Default::default(),
            tag: ::std::default::Default::default(),
            ty: ::std::default::Default::default(),
            vlanaware: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Alias name of the VNet."]
    #[doc = ""]
    pub alias: Option<AliasStr>,
    #[serde(rename = "isolate-ports")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If true, sets the isolated property for all interfaces on the bridge of this VNet."]
    #[doc = ""]
    pub isolate_ports: Option<bool>,
    #[serde(rename = "lock-token")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the token for unlocking the global SDN configuration"]
    #[doc = ""]
    pub lock_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VLAN Tag (for VLAN or QinQ zones) or VXLAN VNI (for VXLAN or EVPN zones)."]
    #[doc = ""]
    pub tag: Option<TagInt>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Type of the VNet."]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow VLANs to pass through this vnet."]
    #[doc = ""]
    pub vlanaware: Option<bool>,
    #[doc = "The SDN vnet object identifier."]
    #[doc = ""]
    pub vnet: String,
    #[doc = "Name of the zone this VNet belongs to."]
    #[doc = ""]
    pub zone: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "State of the SDN configuration object."]
#[doc = ""]
pub enum State {
    #[serde(rename = "changed")]
    Changed,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "new")]
    New,
}
impl TryFrom<&str> for State {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "changed" => Ok(Self::Changed),
            "deleted" => Ok(Self::Deleted),
            "new" => Ok(Self::New),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Type of the VNet."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "vnet")]
    Vnet,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "vnet" => Ok(Self::Vnet),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TagInt(i128);
impl crate::types::bounded_integer::BoundedInteger for TagInt {
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
impl std::convert::TryFrom<i128> for TagInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for TagInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for TagInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AliasStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for AliasStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(256usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("(?^i:[\\(\\)-_.\\w\\d\\s]{0,256})");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"(?^i:[\\(\\)-_.\\w\\d\\s]{0,256})\" and length at most 256";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for AliasStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for AliasStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for AliasStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
impl<T> VnetsClient<T>
where
    T: crate::client::Client,
{
    pub fn vnet(&self, vnet: &str) -> vnet::VnetClient<T> {
        vnet::VnetClient::<T>::new(self.client.clone(), &self.path, vnet)
    }
}
