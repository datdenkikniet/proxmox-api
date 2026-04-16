pub mod fabric_id;
#[derive(Debug, Clone)]
pub struct NodeClient<T> {
    client: T,
    path: String,
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/node"),
        }
    }
}
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN Fabrics Index"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(fabric_id: String, node_id: String, protocol: Protocol) -> Self {
        Self {
            fabric_id,
            node_id,
            protocol,
            digest: ::std::default::Default::default(),
            ip: ::std::default::Default::default(),
            ip6: ::std::default::Default::default(),
            lock_token: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[doc = "Identifier for SDN fabrics"]
    #[doc = ""]
    pub fabric_id: String,
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
    #[doc = "Identifier for nodes in an SDN fabric"]
    #[doc = ""]
    pub node_id: String,
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
impl<T> NodeClient<T>
where
    T: crate::client::Client,
{
    pub fn fabric_id(&self, fabric_id: &str) -> fabric_id::FabricIdClient<T> {
        fabric_id::FabricIdClient::<T>::new(self.client.clone(), &self.path, fabric_id)
    }
}
