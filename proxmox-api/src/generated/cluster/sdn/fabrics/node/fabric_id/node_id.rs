#[derive(Debug, Clone)]
pub struct NodeIdClient<T> {
    client: T,
    path: String,
}
impl<T> NodeIdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, node_id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, node_id),
        }
    }
}
impl<T> NodeIdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Add a node"]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> NodeIdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get a node"]
    #[doc = ""]
    pub fn get(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> NodeIdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update a node"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl PutParams {
    pub fn new(protocol: Protocol) -> Self {
        Self {
            protocol,
            delete: ::std::default::Default::default(),
            digest: ::std::default::Default::default(),
            ip: ::std::default::Default::default(),
            ip6: ::std::default::Default::default(),
            lock_token: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub delete: Vec<Delete>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IPv4 address for this node"]
    #[doc = ""]
    pub ip: Option<::std::net::Ipv4Addr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IPv6 address for this node"]
    #[doc = ""]
    pub ip6: Option<::std::net::Ipv6Addr>,
    #[serde(rename = "lock-token")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the token for unlocking the global SDN configuration"]
    #[doc = ""]
    pub lock_token: Option<String>,
    #[doc = "Type of configuration entry in an SDN Fabric section config"]
    #[doc = ""]
    pub protocol: Protocol,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
pub enum Delete {
    #[serde(rename = "interfaces")]
    Interfaces,
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "ip6")]
    Ip6,
}
impl TryFrom<&str> for Delete {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "interfaces" => Ok(Self::Interfaces),
            "ip" => Ok(Self::Ip),
            "ip6" => Ok(Self::Ip6),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Type of configuration entry in an SDN Fabric section config"]
#[doc = ""]
pub enum Protocol {
    #[serde(rename = "openfabric")]
    Openfabric,
    #[serde(rename = "ospf")]
    Ospf,
}
impl TryFrom<&str> for Protocol {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "openfabric" => Ok(Self::Openfabric),
            "ospf" => Ok(Self::Ospf),
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
