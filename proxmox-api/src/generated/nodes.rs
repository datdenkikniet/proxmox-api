pub mod node;
pub struct NodesClient<T> {
    client: T,
    path: String,
}
impl<T> NodesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "/nodes".to_string(),
        }
    }
}
impl<T> NodesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Cluster node index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(node: String, status: Status) -> Self {
        Self {
            node,
            status,
            cpu: Default::default(),
            level: Default::default(),
            maxcpu: Default::default(),
            maxmem: Default::default(),
            mem: Default::default(),
            ssl_fingerprint: Default::default(),
            uptime: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "CPU utilization."]
    #[doc = ""]
    pub cpu: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Support level."]
    #[doc = ""]
    pub level: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of available CPUs."]
    #[doc = ""]
    pub maxcpu: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of available memory in bytes."]
    #[doc = ""]
    pub maxmem: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Used memory in bytes."]
    #[doc = ""]
    pub mem: Option<u64>,
    #[doc = "The cluster node name."]
    #[doc = ""]
    pub node: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The SSL fingerprint for the node certificate."]
    #[doc = ""]
    pub ssl_fingerprint: Option<String>,
    #[doc = "Node status."]
    #[doc = ""]
    pub status: Status,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node uptime in seconds."]
    #[doc = ""]
    pub uptime: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Node status."]
#[doc = ""]
pub enum Status {
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "unknown")]
    Unknown,
}
impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "offline" => Ok(Self::Offline),
            "online" => Ok(Self::Online),
            "unknown" => Ok(Self::Unknown),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> NodesClient<T>
where
    T: crate::client::Client,
{
    pub fn node(&self, node: &str) -> node::NodeClient<T> {
        node::NodeClient::<T>::new(self.client.clone(), &self.path, node)
    }
}
