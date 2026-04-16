#[derive(Debug, Clone)]
pub struct BridgesClient<T> {
    client: T,
    path: String,
}
impl<T> BridgesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/bridges"),
        }
    }
}
impl<T> BridgesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get a list of all bridges (vnets) that are part of a zone, as well as the ports that are members of that bridge."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(
        name: String,
        ports: Vec<PortsGetOutputItemsPortsItems>,
        vlan_filtering: String,
    ) -> Self {
        Self {
            name,
            ports,
            vlan_filtering,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Name of the bridge."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "All ports that are members of the bridge"]
    #[doc = ""]
    pub ports: Vec<PortsGetOutputItemsPortsItems>,
    #[doc = "Whether VLAN filtering is enabled for this bridge (= VLAN-aware)."]
    #[doc = ""]
    pub vlan_filtering: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PortsGetOutputItemsPortsItems {
    pub fn new(name: String) -> Self {
        Self {
            name,
            index: ::std::default::Default::default(),
            primary_vlan: ::std::default::Default::default(),
            vlans: ::std::default::Default::default(),
            vmid: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PortsGetOutputItemsPortsItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The index of the guests network device that this interface belongs to."]
    #[doc = ""]
    pub index: Option<String>,
    #[doc = "The name of the bridge port."]
    #[doc = ""]
    pub name: String,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The primary VLAN configured for the port of this bridge (= PVID). Only for VLAN-aware bridges."]
    #[doc = ""]
    pub primary_vlan: Option<f64>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of VLANs and VLAN ranges that are allowed for this bridge port in addition to the primary VLAN. Only for VLAN-aware bridges."]
    #[doc = ""]
    pub vlans: Vec<String>,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The ID of the guest that this interface belongs to."]
    #[doc = ""]
    pub vmid: Option<f64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
