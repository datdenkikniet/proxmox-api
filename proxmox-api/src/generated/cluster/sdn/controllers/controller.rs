pub struct ControllerClient<T> {
    client: T,
    path: String,
}
impl<T> ControllerClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, controller: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, controller),
        }
    }
}
impl<T> ControllerClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete sdn controller object configuration."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> ControllerClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read sdn controller configuration."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> ControllerClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update sdn controller object configuration."]
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
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "autonomous system number"]
    #[doc = ""]
    pub asn: Option<u64>,
    #[serde(rename = "bgp-multipath-as-path-relax")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bgp_multipath_as_path_relax: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable ebgp. (remote-as external)"]
    #[doc = ""]
    pub ebgp: Option<bool>,
    #[serde(rename = "ebgp-multihop")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ebgp_multihop: Option<u64>,
    #[serde(rename = "isis-domain")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS domain."]
    #[doc = ""]
    pub isis_domain: Option<String>,
    #[serde(rename = "isis-ifaces")]
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "ISIS interface."]
    #[doc = ""]
    pub isis_ifaces: Vec<String>,
    #[serde(rename = "isis-net")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS network entity title."]
    #[doc = ""]
    pub isis_net: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "source loopback interface."]
    #[doc = ""]
    pub loopback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name."]
    #[doc = ""]
    pub node: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_list",
        deserialize_with = "crate::types::deserialize_list"
    )]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "peers address list."]
    #[doc = ""]
    pub peers: Vec<::std::net::IpAddr>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
