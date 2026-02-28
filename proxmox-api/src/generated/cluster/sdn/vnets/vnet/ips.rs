#[derive(Debug, Clone)]
pub struct IpsClient<T> {
    client: T,
    path: String,
}
impl<T> IpsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ips"),
        }
    }
}
impl<T> IpsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete IP Mappings in a VNet"]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl<T> IpsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create IP Mapping in a VNet"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> IpsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update IP Mapping in a VNet"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl DeleteParams {
    pub fn new(ip: ::std::net::IpAddr, zone: String) -> Self {
        Self {
            ip,
            zone,
            mac: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct DeleteParams {
    #[doc = "The IP address to delete"]
    #[doc = ""]
    pub ip: ::std::net::IpAddr,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Unicast MAC address."]
    #[doc = ""]
    #[doc = "A common MAC address with the I/G (Individual/Group) bit not set."]
    #[doc = ""]
    pub mac: Option<crate::types::MacAddr<false>>,
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
impl PostParams {
    pub fn new(ip: ::std::net::IpAddr, zone: String) -> Self {
        Self {
            ip,
            zone,
            mac: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "The IP address to associate with the given MAC address"]
    #[doc = ""]
    pub ip: ::std::net::IpAddr,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Unicast MAC address."]
    #[doc = ""]
    #[doc = "A common MAC address with the I/G (Individual/Group) bit not set."]
    #[doc = ""]
    pub mac: Option<crate::types::MacAddr<false>>,
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
impl PutParams {
    pub fn new(ip: ::std::net::IpAddr, zone: String) -> Self {
        Self {
            ip,
            zone,
            mac: Default::default(),
            vmid: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[doc = "The IP address to associate with the given MAC address"]
    #[doc = ""]
    pub ip: ::std::net::IpAddr,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Unicast MAC address."]
    #[doc = ""]
    #[doc = "A common MAC address with the I/G (Individual/Group) bit not set."]
    #[doc = ""]
    pub mac: Option<crate::types::MacAddr<false>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The (unique) ID of the VM."]
    #[doc = ""]
    pub vmid: Option<VmidInt>,
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VmidInt(i128);
impl crate::types::bounded_integer::BoundedInteger for VmidInt {
    const MIN: Option<i128> = Some(100i128);
    const MAX: Option<i128> = Some(999999999i128);
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer between 100 and 999999999";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for VmidInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for VmidInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for VmidInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
