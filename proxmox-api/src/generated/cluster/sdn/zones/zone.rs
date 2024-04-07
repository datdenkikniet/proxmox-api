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
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read sdn zone configuration."]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ZoneClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update sdn zone object configuration."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
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
    pub pending: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display running config."]
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
    #[doc = "Advertise evpn subnets if you have silent hosts"]
    pub advertise_subnets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bridge: Option<String>,
    #[serde(rename = "bridge-disable-mac-learning")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable auto mac learning."]
    pub bridge_disable_mac_learning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Frr router name"]
    pub controller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Type of the DHCP backend for this zone"]
    pub dhcp: Option<Dhcp>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(rename = "disable-arp-nd-suppression")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable ipv4 arp && ipv6 neighbour discovery suppression"]
    pub disable_arp_nd_suppression: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "dns api server"]
    pub dns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "dns domain zone  ex: mydomain.com"]
    pub dnszone: Option<String>,
    #[serde(rename = "dp-id")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Faucet dataplane id"]
    pub dp_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of cluster node names."]
    pub exitnodes: Option<String>,
    #[serde(rename = "exitnodes-local-routing")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow exitnodes to connect to evpn guests"]
    pub exitnodes_local_routing: Option<bool>,
    #[serde(rename = "exitnodes-primary")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force traffic to this exitnode first."]
    pub exitnodes_primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use a specific ipam"]
    pub ipam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Anycast logical router mac address"]
    pub mac: Option<crate::types::MacAddr<true>>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU"]
    pub mtu: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of cluster node names."]
    pub nodes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "peers address list."]
    pub peers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "reverse dns api server"]
    pub reversedns: Option<String>,
    #[serde(rename = "rt-import")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Route-Target import"]
    pub rt_import: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Service-VLAN Tag"]
    pub tag: Option<u64>,
    #[serde(rename = "vlan-protocol")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub vlan_protocol: Option<VlanProtocol>,
    #[serde(rename = "vrf-vxlan")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "l3vni."]
    pub vrf_vxlan: Option<u64>,
    #[serde(rename = "vxlan-port")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Vxlan tunnel udp port (default 4789)."]
    pub vxlan_port: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Dhcp {
    #[serde(rename = "dnsmasq")]
    Dnsmasq,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum VlanProtocol {
    #[serde(rename = "802.1ad")]
    _8021ad,
    #[serde(rename = "802.1q")]
    _8021q,
}
impl Default for VlanProtocol {
    fn default() -> Self {
        Self::_8021q
    }
}
