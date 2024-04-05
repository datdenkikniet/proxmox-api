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
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display pending config."]
    pub pending: Option<bool>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display running config."]
    pub running: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
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
impl<T> ControllerClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read sdn controller configuration."]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "autonomous system number"]
    pub asn: Option<()>,
    #[serde(rename = "bgp-multipath-as-path-relax")]
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bgp_multipath_as_path_relax: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable ebgp. (remote-as external)"]
    pub ebgp: Option<bool>,
    #[serde(rename = "ebgp-multihop")]
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ebgp_multihop: Option<u64>,
    #[serde(rename = "isis-domain")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS domain."]
    pub isis_domain: Option<String>,
    #[serde(rename = "isis-ifaces")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS interface."]
    pub isis_ifaces: Option<String>,
    #[serde(rename = "isis-net")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ISIS network entity title."]
    pub isis_net: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "source loopback interface."]
    pub loopback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The cluster node name."]
    pub node: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "peers address list."]
    pub peers: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ControllerClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update sdn controller object configuration."]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
