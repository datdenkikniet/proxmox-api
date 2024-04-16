pub mod ips;
pub mod subnets;
pub struct VnetClient<T> {
    client: T,
    path: String,
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, vnet: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, vnet),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a VnetClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete sdn vnet object configuration."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.delete(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), (), T::Error> for &VnetClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Delete;
    fn exec(&self, params: ()) -> Result<(), T::Error> {
        self.delete()
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read sdn vnet configuration."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<GetParams, GetOutput, T::Error>
    for &VnetClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        self.get(params)
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update sdn vnet object configuration."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PutParams, (), T::Error> for &VnetClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Put;
    fn exec(&self, params: PutParams) -> Result<(), T::Error> {
        self.put(params)
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
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "alias name of the vnet"]
    #[doc = ""]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "vlan or vxlan id"]
    #[doc = ""]
    pub tag: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Allow vm VLANs to pass through this vnet."]
    #[doc = ""]
    pub vlanaware: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "zone id"]
    #[doc = ""]
    pub zone: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    pub fn subnets(&self) -> subnets::SubnetsClient<T> {
        subnets::SubnetsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    pub fn ips(&self) -> ips::IpsClient<T> {
        ips::IpsClient::<T>::new(self.client.clone(), &self.path)
    }
}
