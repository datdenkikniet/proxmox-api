#[derive(Debug, Clone)]
pub struct MacVrfClient<T> {
    client: T,
    path: String,
}
impl<T> MacVrfClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/mac-vrf"),
        }
    }
}
impl<T> MacVrfClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the MAC VRF for a VNet in an EVPN zone."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(
        ip: ::std::net::IpAddr,
        mac: crate::types::MacAddr<true>,
        nexthop: ::std::net::IpAddr,
    ) -> Self {
        Self {
            ip,
            mac,
            nexthop,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The IP address of the MAC VRF entry."]
    #[doc = ""]
    pub ip: ::std::net::IpAddr,
    #[doc = "The MAC address of the MAC VRF entry."]
    #[doc = ""]
    pub mac: crate::types::MacAddr<true>,
    #[doc = "The IP address of the nexthop."]
    #[doc = ""]
    pub nexthop: ::std::net::IpAddr,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
