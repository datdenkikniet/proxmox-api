#[derive(Debug, Clone)]
pub struct AllClient<T> {
    client: T,
    path: String,
}
impl<T> AllClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/all"),
        }
    }
}
impl<T> AllClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN Fabrics Index"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl FabricsGetOutputFabricsItems {
    pub fn new(id: String, protocol: Protocol) -> Self {
        Self {
            id,
            protocol,
            area: ::std::default::Default::default(),
            csnp_interval: ::std::default::Default::default(),
            digest: ::std::default::Default::default(),
            hello_interval: ::std::default::Default::default(),
            ip6_prefix: ::std::default::Default::default(),
            ip_prefix: ::std::default::Default::default(),
            lock_token: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct FabricsGetOutputFabricsItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OSPF area. Either a IPv4 address or a 32-bit number. Gets validated in rust."]
    #[doc = ""]
    pub area: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The csnp_interval property for Openfabric"]
    #[doc = ""]
    pub csnp_interval: Option<CsnpIntervalNum>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<DigestStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The hello_interval property for Openfabric"]
    #[doc = ""]
    pub hello_interval: Option<HelloIntervalNum>,
    #[doc = "Identifier for SDN fabrics"]
    #[doc = ""]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The IP prefix for Node IPs"]
    #[doc = ""]
    pub ip6_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The IP prefix for Node IPs"]
    #[doc = ""]
    pub ip_prefix: Option<String>,
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
impl GetOutput {
    pub fn new(
        fabrics: Vec<FabricsGetOutputFabricsItems>,
        nodes: Vec<NodesGetOutputNodesItems>,
    ) -> Self {
        Self {
            fabrics,
            nodes,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub fabrics: Vec<FabricsGetOutputFabricsItems>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub nodes: Vec<NodesGetOutputNodesItems>,
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
impl NodesGetOutputNodesItems {
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
pub struct NodesGetOutputNodesItems {
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CsnpIntervalNum(f64);
impl crate::types::bounded_number::BoundedNumber for CsnpIntervalNum {
    const MIN: Option<f64> = Some(1f64);
    const MAX: Option<f64> = Some(600f64);
    const DEFAULT: Option<f64> = None::<f64>;
    const TYPE_DESCRIPTION: &'static str = "an number between 1 and 600";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for CsnpIntervalNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for CsnpIntervalNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for CsnpIntervalNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for CsnpIntervalNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for CsnpIntervalNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_number::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CsnpIntervalNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_number::deserialize_bounded_number(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HelloIntervalNum(f64);
impl crate::types::bounded_number::BoundedNumber for HelloIntervalNum {
    const MIN: Option<f64> = Some(1f64);
    const MAX: Option<f64> = Some(600f64);
    const DEFAULT: Option<f64> = None::<f64>;
    const TYPE_DESCRIPTION: &'static str = "an number between 1 and 600";
    fn get(&self) -> f64 {
        self.0
    }
    fn new(value: f64) -> Result<Self, crate::types::bounded_number::BoundedNumberError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<f64> for HelloIntervalNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value)
    }
}
impl std::convert::TryFrom<f32> for HelloIntervalNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i32> for HelloIntervalNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl std::convert::TryFrom<i64> for HelloIntervalNum {
    type Error = crate::types::bounded_number::BoundedNumberError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        crate::types::bounded_number::BoundedNumber::new(value as f64)
    }
}
impl ::serde::Serialize for HelloIntervalNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_number::serialize_bounded_number(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for HelloIntervalNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_number::deserialize_bounded_number(deserializer)
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
