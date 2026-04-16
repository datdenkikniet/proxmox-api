pub mod zone;
#[derive(Debug, Clone)]
pub struct ZonesClient<T> {
    client: T,
    path: String,
}
impl<T> ZonesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/zones"),
        }
    }
}
impl<T> ZonesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "SDN zones index."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ZonesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new sdn zone object."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(ty: Type, zone: String) -> Self {
        Self {
            ty,
            zone,
            advertise_subnets: ::std::default::Default::default(),
            bridge: ::std::default::Default::default(),
            bridge_disable_mac_learning: ::std::default::Default::default(),
            controller: ::std::default::Default::default(),
            dhcp: ::std::default::Default::default(),
            digest: ::std::default::Default::default(),
            disable_arp_nd_suppression: ::std::default::Default::default(),
            dns: ::std::default::Default::default(),
            dnszone: ::std::default::Default::default(),
            exitnodes: ::std::default::Default::default(),
            exitnodes_local_routing: ::std::default::Default::default(),
            exitnodes_primary: ::std::default::Default::default(),
            ipam: ::std::default::Default::default(),
            mac: ::std::default::Default::default(),
            mtu: ::std::default::Default::default(),
            nodes: ::std::default::Default::default(),
            peers: ::std::default::Default::default(),
            pending: ::std::default::Default::default(),
            reversedns: ::std::default::Default::default(),
            rt_import: ::std::default::Default::default(),
            state: ::std::default::Default::default(),
            tag: ::std::default::Default::default(),
            vlan_protocol: ::std::default::Default::default(),
            vrf_vxlan: ::std::default::Default::default(),
            vxlan_port: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(rename = "advertise-subnets")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Advertise IP prefixes (Type-5 routes) instead of MAC/IP pairs (Type-2 routes). EVPN zone only."]
    #[doc = ""]
    pub advertise_subnets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the bridge for which VLANs should be managed. VLAN & QinQ zone only."]
    #[doc = ""]
    pub bridge: Option<String>,
    #[serde(rename = "bridge-disable-mac-learning")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable auto mac learning. VLAN zone only."]
    #[doc = ""]
    pub bridge_disable_mac_learning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ID of the controller for this zone. EVPN zone only."]
    #[doc = ""]
    pub controller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of DHCP server backend for this zone."]
    #[doc = ""]
    pub dhcp: Option<Dhcp>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Digest of the controller section."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(rename = "disable-arp-nd-suppression")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Suppress IPv4 ARP && IPv6 Neighbour Discovery messages. EVPN zone only."]
    #[doc = ""]
    pub disable_arp_nd_suppression: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ID of the DNS server for this zone."]
    #[doc = ""]
    pub dns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Domain name for this zone."]
    #[doc = ""]
    pub dnszone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of PVE Nodes that should act as exit node for this zone. EVPN zone only."]
    #[doc = ""]
    pub exitnodes: Option<String>,
    #[serde(rename = "exitnodes-local-routing")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create routes on the exit nodes, so they can connect to EVPN guests. EVPN zone only."]
    #[doc = ""]
    pub exitnodes_local_routing: Option<bool>,
    #[serde(rename = "exitnodes-primary")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force traffic through this exitnode first. EVPN zone only."]
    #[doc = ""]
    pub exitnodes_primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ID of the IPAM for this zone."]
    #[doc = ""]
    pub ipam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MAC address of the anycast router for this zone."]
    #[doc = ""]
    pub mac: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU of the zone, will be used for the created VNet bridges."]
    #[doc = ""]
    pub mtu: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Nodes where this zone should be created."]
    #[doc = ""]
    pub nodes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comma-separated list of peers, that are part of the VXLAN zone. Usually the IPs of the nodes. VXLAN zone only."]
    #[doc = ""]
    pub peers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Changes that have not yet been applied to the running configuration."]
    #[doc = ""]
    pub pending: Option<PendingGetOutputItemsPending>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ID of the reverse DNS server for this zone."]
    #[doc = ""]
    pub reversedns: Option<String>,
    #[serde(rename = "rt-import")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Route-Targets that should be imported into the VRF of this zone via BGP. EVPN zone only."]
    #[doc = ""]
    pub rt_import: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "State of the SDN configuration object."]
    #[doc = ""]
    pub state: Option<State>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Service-VLAN Tag (outer VLAN). QinQ zone only"]
    #[doc = ""]
    pub tag: Option<u64>,
    #[serde(rename = "type")]
    #[doc = "Type of the zone."]
    #[doc = ""]
    pub ty: Type,
    #[serde(rename = "vlan-protocol")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VLAN protocol for the creation of the QinQ zone. QinQ zone only."]
    #[doc = ""]
    pub vlan_protocol: Option<VlanProtocol>,
    #[serde(rename = "vrf-vxlan")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VNI for the zone VRF. EVPN zone only."]
    #[doc = ""]
    pub vrf_vxlan: Option<VrfVxlanInt>,
    #[serde(rename = "vxlan-port")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "UDP port that should be used for the VXLAN tunnel (default 4789). VXLAN zone only."]
    #[doc = ""]
    pub vxlan_port: Option<VxlanPortInt>,
    #[doc = "Name of the zone."]
    #[doc = ""]
    pub zone: String,
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
    #[doc = "Only list SDN zones of specific type"]
    #[doc = ""]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PendingGetOutputItemsPending {
    #[serde(rename = "advertise-subnets")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Advertise IP prefixes (Type-5 routes) instead of MAC/IP pairs (Type-2 routes). EVPN zone only."]
    #[doc = ""]
    pub advertise_subnets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the bridge for which VLANs should be managed. VLAN & QinQ zone only."]
    #[doc = ""]
    pub bridge: Option<String>,
    #[serde(rename = "bridge-disable-mac-learning")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable auto mac learning. VLAN zone only."]
    #[doc = ""]
    pub bridge_disable_mac_learning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ID of the controller for this zone. EVPN zone only."]
    #[doc = ""]
    pub controller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Name of DHCP server backend for this zone."]
    #[doc = ""]
    pub dhcp: Option<Dhcp>,
    #[serde(rename = "disable-arp-nd-suppression")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Suppress IPv4 ARP && IPv6 Neighbour Discovery messages. EVPN zone only."]
    #[doc = ""]
    pub disable_arp_nd_suppression: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ID of the DNS server for this zone."]
    #[doc = ""]
    pub dns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Domain name for this zone."]
    #[doc = ""]
    pub dnszone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of PVE Nodes that should act as exit node for this zone. EVPN zone only."]
    #[doc = ""]
    pub exitnodes: Option<String>,
    #[serde(rename = "exitnodes-local-routing")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Create routes on the exit nodes, so they can connect to EVPN guests. EVPN zone only."]
    #[doc = ""]
    pub exitnodes_local_routing: Option<bool>,
    #[serde(rename = "exitnodes-primary")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force traffic through this exitnode first. EVPN zone only."]
    #[doc = ""]
    pub exitnodes_primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ID of the IPAM for this zone."]
    #[doc = ""]
    pub ipam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MAC address of the anycast router for this zone."]
    #[doc = ""]
    pub mac: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU of the zone, will be used for the created VNet bridges."]
    #[doc = ""]
    pub mtu: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Nodes where this zone should be created."]
    #[doc = ""]
    pub nodes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comma-separated list of peers, that are part of the VXLAN zone. Usually the IPs of the nodes. VXLAN zone only."]
    #[doc = ""]
    pub peers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ID of the reverse DNS server for this zone."]
    #[doc = ""]
    pub reversedns: Option<String>,
    #[serde(rename = "rt-import")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Route-Targets that should be imported into the VRF of this zone via BGP. EVPN zone only."]
    #[doc = ""]
    pub rt_import: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Service-VLAN Tag (outer VLAN). QinQ zone only"]
    #[doc = ""]
    pub tag: Option<u64>,
    #[serde(rename = "vlan-protocol")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VLAN protocol for the creation of the QinQ zone. QinQ zone only."]
    #[doc = ""]
    pub vlan_protocol: Option<VlanProtocol>,
    #[serde(rename = "vrf-vxlan")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VNI for the zone VRF. EVPN zone only."]
    #[doc = ""]
    pub vrf_vxlan: Option<VrfVxlanInt>,
    #[serde(rename = "vxlan-port")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "UDP port that should be used for the VXLAN tunnel (default 4789). VXLAN zone only."]
    #[doc = ""]
    pub vxlan_port: Option<VxlanPortInt>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(ty: Type, zone: String) -> Self {
        Self {
            ty,
            zone,
            advertise_subnets: ::std::default::Default::default(),
            bridge: ::std::default::Default::default(),
            bridge_disable_mac_learning: ::std::default::Default::default(),
            controller: ::std::default::Default::default(),
            dhcp: ::std::default::Default::default(),
            disable_arp_nd_suppression: ::std::default::Default::default(),
            dns: ::std::default::Default::default(),
            dnszone: ::std::default::Default::default(),
            dp_id: ::std::default::Default::default(),
            exitnodes: ::std::default::Default::default(),
            exitnodes_local_routing: ::std::default::Default::default(),
            exitnodes_primary: ::std::default::Default::default(),
            fabric: ::std::default::Default::default(),
            ipam: ::std::default::Default::default(),
            lock_token: ::std::default::Default::default(),
            mac: ::std::default::Default::default(),
            mtu: ::std::default::Default::default(),
            nodes: ::std::default::Default::default(),
            peers: ::std::default::Default::default(),
            reversedns: ::std::default::Default::default(),
            rt_import: ::std::default::Default::default(),
            tag: ::std::default::Default::default(),
            vlan_protocol: ::std::default::Default::default(),
            vrf_vxlan: ::std::default::Default::default(),
            vxlan_port: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(rename = "advertise-subnets")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Advertise IP prefixes (Type-5 routes) instead of MAC/IP pairs (Type-2 routes)."]
    #[doc = ""]
    pub advertise_subnets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The bridge for which VLANs should be managed."]
    #[doc = ""]
    pub bridge: Option<String>,
    #[serde(rename = "bridge-disable-mac-learning")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable auto mac learning."]
    #[doc = ""]
    pub bridge_disable_mac_learning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Controller for this zone."]
    #[doc = ""]
    pub controller: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Type of the DHCP backend for this zone"]
    #[doc = ""]
    pub dhcp: Option<Dhcp>,
    #[serde(rename = "disable-arp-nd-suppression")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Suppress IPv4 ARP && IPv6 Neighbour Discovery messages."]
    #[doc = ""]
    pub disable_arp_nd_suppression: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "dns api server"]
    #[doc = ""]
    pub dns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "dns domain zone  ex: mydomain.com"]
    #[doc = ""]
    pub dnszone: Option<String>,
    #[serde(rename = "dp-id")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Faucet dataplane id"]
    #[doc = ""]
    pub dp_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of cluster node names."]
    #[doc = ""]
    pub exitnodes: Option<String>,
    #[serde(rename = "exitnodes-local-routing")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow exitnodes to connect to EVPN guests."]
    #[doc = ""]
    pub exitnodes_local_routing: Option<bool>,
    #[serde(rename = "exitnodes-primary")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force traffic through this exitnode first."]
    #[doc = ""]
    pub exitnodes_primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "SDN fabric to use as underlay for this VXLAN zone."]
    #[doc = ""]
    pub fabric: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "use a specific ipam"]
    #[doc = ""]
    pub ipam: Option<String>,
    #[serde(rename = "lock-token")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the token for unlocking the global SDN configuration"]
    #[doc = ""]
    pub lock_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Anycast logical router mac address."]
    #[doc = ""]
    pub mac: Option<crate::types::MacAddr<true>>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU of the zone, will be used for the created VNet bridges."]
    #[doc = ""]
    pub mtu: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of cluster node names."]
    #[doc = ""]
    pub nodes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comma-separated list of peers, that are part of the VXLAN zone. Usually the IPs of the nodes."]
    #[doc = ""]
    pub peers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "reverse dns api server"]
    #[doc = ""]
    pub reversedns: Option<String>,
    #[serde(rename = "rt-import")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of Route Targets that should be imported into the VRF of the zone."]
    #[doc = ""]
    pub rt_import: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_unsigned_int_optional",
        deserialize_with = "crate::types::deserialize_unsigned_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Service-VLAN Tag (outer VLAN)"]
    #[doc = ""]
    pub tag: Option<u64>,
    #[serde(rename = "type")]
    #[doc = "Plugin type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(rename = "vlan-protocol")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Which VLAN protocol should be used for the creation of the QinQ zone."]
    #[doc = ""]
    pub vlan_protocol: Option<VlanProtocol>,
    #[serde(rename = "vrf-vxlan")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "VNI for the zone VRF."]
    #[doc = ""]
    pub vrf_vxlan: Option<VrfVxlanInt>,
    #[serde(rename = "vxlan-port")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "UDP port that should be used for the VXLAN tunnel (default 4789)."]
    #[doc = ""]
    pub vxlan_port: Option<VxlanPortInt>,
    #[doc = "The SDN zone object identifier."]
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
#[doc = "Name of DHCP server backend for this zone."]
#[doc = ""]
pub enum Dhcp {
    #[serde(rename = "dnsmasq")]
    Dnsmasq,
}
impl TryFrom<&str> for Dhcp {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "dnsmasq" => Ok(Self::Dnsmasq),
            v => Err(format!("Unknown variant {v}")),
        }
    }
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
#[doc = "Only list SDN zones of specific type"]
#[doc = ""]
pub enum Type {
    #[serde(rename = "evpn")]
    Evpn,
    #[serde(rename = "faucet")]
    Faucet,
    #[serde(rename = "qinq")]
    Qinq,
    #[serde(rename = "simple")]
    Simple,
    #[serde(rename = "vlan")]
    Vlan,
    #[serde(rename = "vxlan")]
    Vxlan,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "evpn" => Ok(Self::Evpn),
            "faucet" => Ok(Self::Faucet),
            "qinq" => Ok(Self::Qinq),
            "simple" => Ok(Self::Simple),
            "vlan" => Ok(Self::Vlan),
            "vxlan" => Ok(Self::Vxlan),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq, Default)]
#[doc = "VLAN protocol for the creation of the QinQ zone. QinQ zone only."]
#[doc = ""]
pub enum VlanProtocol {
    #[serde(rename = "802.1ad")]
    _8021ad,
    #[serde(rename = "802.1q")]
    #[default]
    _8021q,
}
impl TryFrom<&str> for VlanProtocol {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "802.1ad" => Ok(Self::_8021ad),
            "802.1q" => Ok(Self::_8021q),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VrfVxlanInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VrfVxlanInt {
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
impl std::convert::TryFrom<i128> for VrfVxlanInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VrfVxlanInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VrfVxlanInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VxlanPortInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VxlanPortInt {
    const MIN: Option<i128> = Some(1i128);
    const MAX: Option<i128> = Some(65536i128);
    const DEFAULT: Option<i128> = Some(4789i128);
    const TYPE_DESCRIPTION: &'static str = "an integer between 1 and 65536";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VxlanPortInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VxlanPortInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_integer::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VxlanPortInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_integer::deserialize_bounded_integer(deserializer)
    }
}
impl<T> ZonesClient<T>
where
    T: crate::client::Client,
{
    pub fn zone(&self, zone: &str) -> zone::ZoneClient<T> {
        zone::ZoneClient::<T>::new(self.client.clone(), &self.path, zone)
    }
}
