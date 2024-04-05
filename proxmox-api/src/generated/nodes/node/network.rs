pub mod iface;
pub struct NetworkClient<T> {
    client: T,
    path: String,
}
impl<T> NetworkClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/network"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum BondMode {
    #[serde(rename = "802.3ad")]
    _8023ad,
    #[serde(rename = "active-backup")]
    ActiveBackup,
    #[serde(rename = "balance-alb")]
    BalanceAlb,
    #[serde(rename = "balance-rr")]
    BalanceRr,
    #[serde(rename = "balance-slb")]
    BalanceSlb,
    #[serde(rename = "balance-tlb")]
    BalanceTlb,
    #[serde(rename = "balance-xor")]
    BalanceXor,
    #[serde(rename = "broadcast")]
    Broadcast,
    #[serde(rename = "lacp-balance-slb")]
    LacpBalanceSlb,
    #[serde(rename = "lacp-balance-tcp")]
    LacpBalanceTcp,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum BondXmitHashPolicy {
    #[serde(rename = "layer2")]
    Layer2,
    #[serde(rename = "layer2+3")]
    Layer23,
    #[serde(rename = "layer3+4")]
    Layer34,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    OVSBond,
    OVSBridge,
    OVSIntPort,
    OVSPort,
    #[serde(rename = "alias")]
    Alias,
    #[serde(rename = "any_bridge")]
    AnyBridge,
    #[serde(rename = "any_local_bridge")]
    AnyLocalBridge,
    #[serde(rename = "bond")]
    Bond,
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "eth")]
    Eth,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "vlan")]
    Vlan,
}
impl<T> NetworkClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Revert network configuration changes."]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list specific interface types."]
    pub ty: Option<Type>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> NetworkClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List available networks"]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl PostParams {
    pub fn new(iface: String, ty: Type) -> Self {
        Self {
            iface,
            ty,
            address: Default::default(),
            address6: Default::default(),
            autostart: Default::default(),
            bond_primary: Default::default(),
            bond_mode: Default::default(),
            bond_xmit_hash_policy: Default::default(),
            bridge_ports: Default::default(),
            bridge_vlan_aware: Default::default(),
            cidr: Default::default(),
            cidr6: Default::default(),
            comments: Default::default(),
            comments6: Default::default(),
            gateway: Default::default(),
            gateway6: Default::default(),
            mtu: Default::default(),
            netmask: Default::default(),
            netmask6: Default::default(),
            ovs_bonds: Default::default(),
            ovs_bridge: Default::default(),
            ovs_options: Default::default(),
            ovs_ports: Default::default(),
            ovs_tag: Default::default(),
            slaves: Default::default(),
            vlan_id: Default::default(),
            vlan_raw_device: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP address."]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP address."]
    pub address6: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Automatically start interface on boot."]
    pub autostart: Option<bool>,
    #[serde(rename = "bond-primary")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the primary interface for active-backup bond."]
    pub bond_primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Bonding mode."]
    pub bond_mode: Option<BondMode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Selects the transmit hash policy to use for slave selection in balance-xor and 802.3ad modes."]
    pub bond_xmit_hash_policy: Option<BondXmitHashPolicy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the interfaces you want to add to your bridge."]
    pub bridge_ports: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable bridge vlan support."]
    pub bridge_vlan_aware: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IPv4 CIDR."]
    pub cidr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IPv6 CIDR."]
    pub cidr6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comments"]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comments"]
    pub comments6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default gateway address."]
    pub gateway: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default ipv6 gateway address."]
    pub gateway6: Option<String>,
    #[doc = "Network interface name."]
    pub iface: String,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU."]
    pub mtu: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Network mask."]
    pub netmask: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Network mask."]
    pub netmask6: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the interfaces used by the bonding device."]
    pub ovs_bonds: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The OVS bridge associated with a OVS port. This is required when you create an OVS port."]
    pub ovs_bridge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OVS interface options."]
    pub ovs_options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the interfaces you want to add to your bridge."]
    pub ovs_ports: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify a VLan tag (used by OVSPort, OVSIntPort, OVSBond)"]
    pub ovs_tag: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the interfaces used by the bonding device."]
    pub slaves: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Network interface type"]
    pub ty: Type,
    #[serde(rename = "vlan-id")]
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "vlan-id for a custom named vlan interface (ifupdown2 only)."]
    pub vlan_id: Option<u64>,
    #[serde(rename = "vlan-raw-device")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the raw interface for the vlan interface."]
    pub vlan_raw_device: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> NetworkClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create network device configuration"]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> NetworkClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Reload network configuration"]
    pub fn put(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &())
    }
}
impl<T> NetworkClient<T>
where
    T: crate::client::Client,
{
    pub fn iface(&self, iface: &str) -> iface::IfaceClient<T> {
        iface::IfaceClient::<T>::new(self.client.clone(), &self.path, iface)
    }
}