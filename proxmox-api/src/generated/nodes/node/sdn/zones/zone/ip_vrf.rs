#[derive(Debug, Clone)]
pub struct IpVrfClient<T> {
    client: T,
    path: String,
}
impl<T> IpVrfClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ip-vrf"),
        }
    }
}
impl<T> IpVrfClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the IP VRF of an EVPN zone."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(ip: String, metric: i64, nexthops: Vec<String>, protocol: String) -> Self {
        Self {
            ip,
            metric,
            nexthops,
            protocol,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The CIDR of the route table entry."]
    #[doc = ""]
    pub ip: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "This route's metric."]
    #[doc = ""]
    pub metric: i64,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of nexthops for the route table entry."]
    #[doc = ""]
    pub nexthops: Vec<String>,
    #[doc = "The protocol where this route was learned from (e.g. BGP)."]
    #[doc = ""]
    pub protocol: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
