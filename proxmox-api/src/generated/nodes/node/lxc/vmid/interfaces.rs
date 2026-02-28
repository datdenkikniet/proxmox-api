#[derive(Debug, Clone)]
pub struct InterfacesClient<T> {
    client: T,
    path: String,
}
impl<T> InterfacesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/interfaces"),
        }
    }
}
impl<T> InterfacesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get IP addresses of the specified container interface."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(
        hardware_address: String,
        hwaddr: String,
        ip_addresses: Vec<IpAddressesGetOutputItemsIpAddressesItems>,
        name: String,
    ) -> Self {
        Self {
            hardware_address,
            hwaddr,
            ip_addresses,
            name,
            inet: Default::default(),
            inet6: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(rename = "hardware-address")]
    #[doc = "The MAC address of the interface"]
    #[doc = ""]
    pub hardware_address: String,
    #[doc = "The MAC address of the interface"]
    #[doc = ""]
    pub hwaddr: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The IPv4 address of the interface"]
    #[doc = ""]
    pub inet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The IPv6 address of the interface"]
    #[doc = ""]
    pub inet6: Option<String>,
    #[serde(rename = "ip-addresses")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "The addresses of the interface"]
    #[doc = ""]
    pub ip_addresses: Vec<IpAddressesGetOutputItemsIpAddressesItems>,
    #[doc = "The name of the interface"]
    #[doc = ""]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct IpAddressesGetOutputItemsIpAddressesItems {
    #[serde(rename = "ip-address")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP-Address"]
    #[doc = ""]
    pub ip_address: Option<String>,
    #[serde(rename = "ip-address-type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP-Family"]
    #[doc = ""]
    pub ip_address_type: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "IP-Prefix"]
    #[doc = ""]
    pub prefix: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
