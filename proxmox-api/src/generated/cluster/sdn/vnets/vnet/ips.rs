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
impl<'a, T> crate::ProxmoxClient for &'a IpsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> IpsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete IP Mappings in a VNet"]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.delete(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<DeleteParams, (), T::Error> for &IpsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Delete;
    fn exec(&self, params: DeleteParams) -> Result<(), T::Error> {
        self.delete(params)
    }
}
impl<T> IpsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create IP Mapping in a VNet"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, (), T::Error> for &IpsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<(), T::Error> {
        self.post(params)
    }
}
impl<T> IpsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update IP Mapping in a VNet"]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PutParams, (), T::Error> for &IpsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Put;
    fn exec(&self, params: PutParams) -> Result<(), T::Error> {
        self.put(params)
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
    pub vmid: Option<crate::types::VmId>,
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
