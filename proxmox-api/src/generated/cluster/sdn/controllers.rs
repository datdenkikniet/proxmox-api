pub mod controller;
#[derive(Debug, Clone)]
pub struct ControllersClient<T> {
    client: T,
    path: String,
}
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/controllers"),
        }
    }
}
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN controllers index."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new sdn controller object."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(controller: String, ty: String) -> Self {
        Self {
            controller,
            ty,
            pending: Default::default(),
            state: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub controller: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub state: Option<String>,
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
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list sdn controllers of specific type"]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(controller: String, ty: Type) -> Self {
        Self {
            controller,
            ty,
            asn: Default::default(),
            bgp_multipath_as_path_relax: Default::default(),
            ebgp: Default::default(),
            ebgp_multihop: Default::default(),
            isis_domain: Default::default(),
            isis_ifaces: Default::default(),
            isis_net: Default::default(),
            loopback: Default::default(),
            node: Default::default(),
            peers: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "autonomous system number"]
    #[doc = ""]
    pub asn: Option<AsnInt>,
    #[serde(rename = "bgp-multipath-as-path-relax")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bgp_multipath_as_path_relax: Option<bool>,
    #[doc = "The SDN controller object identifier."]
    #[doc = ""]
    pub controller: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable ebgp. (remote-as external)"]
    #[doc = ""]
    pub ebgp: Option<bool>,
    #[serde(rename = "ebgp-multihop")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ebgp_multihop: Option<i64>,
    #[serde(rename = "isis-domain")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS domain."]
    #[doc = ""]
    pub isis_domain: Option<String>,
    #[serde(rename = "isis-ifaces")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS interface."]
    #[doc = ""]
    pub isis_ifaces: Option<String>,
    #[serde(rename = "isis-net")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS network entity title."]
    #[doc = ""]
    pub isis_net: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "source loopback interface."]
    #[doc = ""]
    pub loopback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name."]
    #[doc = ""]
    pub node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "peers address list."]
    #[doc = ""]
    pub peers: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Plugin type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Only list sdn controllers of specific type"]
#[doc = ""]
pub enum Type {
    #[serde(rename = "bgp")]
    Bgp,
    #[serde(rename = "evpn")]
    Evpn,
    #[serde(rename = "faucet")]
    Faucet,
    #[serde(rename = "isis")]
    Isis,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "bgp" => Ok(Self::Bgp),
            "evpn" => Ok(Self::Evpn),
            "faucet" => Ok(Self::Faucet),
            "isis" => Ok(Self::Isis),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AsnInt(i128);
impl crate::types::bounded_integer::BoundedInteger for AsnInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = Some(4294967296i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 0 and 4294967296";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for AsnInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for AsnInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for AsnInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    pub fn controller(&self, controller: &str) -> controller::ControllerClient<T> {
        controller::ControllerClient::<T>::new(self.client.clone(), &self.path, controller)
    }
}
