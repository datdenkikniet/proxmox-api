pub mod ips;
pub mod subnets;
#[derive(Debug, Clone)]
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
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete sdn vnet object configuration."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read sdn vnet configuration."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> VnetClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update sdn vnet object configuration."]
    #[doc = ""]
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
    pub tag: Option<i64>,
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
