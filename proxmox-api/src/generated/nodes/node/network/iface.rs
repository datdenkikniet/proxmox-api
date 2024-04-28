#[derive(Debug, Clone)]
pub struct IfaceClient<T> {
    client: T,
    path: String,
}
impl<T> IfaceClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, iface: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, iface),
        }
    }
}
impl<T> IfaceClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete network device configuration"]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        match self.client.delete(&path, &()) {
            Ok(o) => Ok(o),
            Err(e) if crate::client::Error::is_empty_data(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
impl<T> IfaceClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read network device configuration"]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> IfaceClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update network device configuration"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        match self.client.put(&path, &params) {
            Ok(o) => Ok(o),
            Err(e) if crate::client::Error::is_empty_data(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
impl GetOutput {
    pub fn new(method: String, ty: String) -> Self {
        Self {
            method,
            ty,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    pub method: String,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PutParams {
    pub fn new(ty: Type) -> Self {
        Self {
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
            delete: Default::default(),
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
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP address."]
    #[doc = ""]
    pub address: Option<::std::net::Ipv4Addr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP address."]
    #[doc = ""]
    pub address6: Option<::std::net::Ipv6Addr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Automatically start interface on boot."]
    #[doc = ""]
    pub autostart: Option<bool>,
    #[serde(rename = "bond-primary")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the primary interface for active-backup bond."]
    #[doc = ""]
    pub bond_primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Bonding mode."]
    #[doc = ""]
    pub bond_mode: Option<BondMode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Selects the transmit hash policy to use for slave selection in balance-xor and 802.3ad modes."]
    #[doc = ""]
    pub bond_xmit_hash_policy: Option<BondXmitHashPolicy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the interfaces you want to add to your bridge."]
    #[doc = ""]
    pub bridge_ports: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable bridge vlan support."]
    #[doc = ""]
    pub bridge_vlan_aware: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IPv4 CIDR."]
    #[doc = ""]
    pub cidr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IPv6 CIDR."]
    #[doc = ""]
    pub cidr6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comments"]
    #[doc = ""]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Comments"]
    #[doc = ""]
    pub comments6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default gateway address."]
    #[doc = ""]
    pub gateway: Option<::std::net::Ipv4Addr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Default ipv6 gateway address."]
    #[doc = ""]
    pub gateway6: Option<::std::net::Ipv6Addr>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "MTU."]
    #[doc = ""]
    pub mtu: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Network mask."]
    #[doc = ""]
    pub netmask: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Network mask."]
    #[doc = ""]
    pub netmask6: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the interfaces used by the bonding device."]
    #[doc = ""]
    pub ovs_bonds: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The OVS bridge associated with a OVS port. This is required when you create an OVS port."]
    #[doc = ""]
    pub ovs_bridge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "OVS interface options."]
    #[doc = ""]
    pub ovs_options: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the interfaces you want to add to your bridge."]
    #[doc = ""]
    pub ovs_ports: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify a VLan tag (used by OVSPort, OVSIntPort, OVSBond)"]
    #[doc = ""]
    pub ovs_tag: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the interfaces used by the bonding device."]
    #[doc = ""]
    pub slaves: Option<String>,
    #[serde(rename = "type")]
    #[doc = "Network interface type"]
    #[doc = ""]
    pub ty: Type,
    #[serde(rename = "vlan-id")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "vlan-id for a custom named vlan interface (ifupdown2 only)."]
    #[doc = ""]
    pub vlan_id: Option<i64>,
    #[serde(rename = "vlan-raw-device")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Specify the raw interface for the vlan interface."]
    #[doc = ""]
    pub vlan_raw_device: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Bonding mode."]
#[doc = ""]
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
impl TryFrom<&str> for BondMode {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "802.3ad" => Ok(Self::_8023ad),
            "active-backup" => Ok(Self::ActiveBackup),
            "balance-alb" => Ok(Self::BalanceAlb),
            "balance-rr" => Ok(Self::BalanceRr),
            "balance-slb" => Ok(Self::BalanceSlb),
            "balance-tlb" => Ok(Self::BalanceTlb),
            "balance-xor" => Ok(Self::BalanceXor),
            "broadcast" => Ok(Self::Broadcast),
            "lacp-balance-slb" => Ok(Self::LacpBalanceSlb),
            "lacp-balance-tcp" => Ok(Self::LacpBalanceTcp),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Selects the transmit hash policy to use for slave selection in balance-xor and 802.3ad modes."]
#[doc = ""]
pub enum BondXmitHashPolicy {
    #[serde(rename = "layer2")]
    Layer2,
    #[serde(rename = "layer2+3")]
    Layer23,
    #[serde(rename = "layer3+4")]
    Layer34,
}
impl TryFrom<&str> for BondXmitHashPolicy {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "layer2" => Ok(Self::Layer2),
            "layer2+3" => Ok(Self::Layer23),
            "layer3+4" => Ok(Self::Layer34),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Network interface type"]
#[doc = ""]
pub enum Type {
    OVSBond,
    OVSBridge,
    OVSIntPort,
    OVSPort,
    #[serde(rename = "alias")]
    Alias,
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
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "OVSBond" => Ok(Self::OVSBond),
            "OVSBridge" => Ok(Self::OVSBridge),
            "OVSIntPort" => Ok(Self::OVSIntPort),
            "OVSPort" => Ok(Self::OVSPort),
            "alias" => Ok(Self::Alias),
            "bond" => Ok(Self::Bond),
            "bridge" => Ok(Self::Bridge),
            "eth" => Ok(Self::Eth),
            "unknown" => Ok(Self::Unknown),
            "vlan" => Ok(Self::Vlan),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
