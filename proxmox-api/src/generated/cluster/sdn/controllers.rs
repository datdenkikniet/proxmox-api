pub mod controller;
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
            state: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub controller: String,
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
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "autonomous system number"]
    #[doc = ""]
    pub asn: Option<u64>,
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
    pub ebgp_multihop: Option<u64>,
    #[serde(rename = "isis-domain")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS domain."]
    #[doc = ""]
    pub isis_domain: Option<String>,
    #[serde(rename = "isis-ifaces")]
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "ISIS interface."]
    #[doc = ""]
    pub isis_ifaces: Vec<String>,
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
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "peers address list."]
    #[doc = ""]
    pub peers: Vec<::std::net::IpAddr>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
impl<T> ControllersClient<T>
where
    T: crate::client::Client,
{
    pub fn controller(&self, controller: &str) -> controller::ControllerClient<T> {
        controller::ControllerClient::<T>::new(self.client.clone(), &self.path, controller)
    }
}
