#[derive(Debug, Clone)]
pub struct InitClient<T> {
    client: T,
    path: String,
}
impl<T> InitClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/init"),
        }
    }
}
impl<T> InitClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create initial ceph default configuration and setup symlinks."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "cluster-network")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Declare a separate cluster network, OSDs will routeheartbeat, object replication and recovery traffic over it"]
    #[doc = ""]
    pub cluster_network: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Disable cephx authentication."]
    #[doc = ""]
    #[doc = "WARNING: cephx is a security feature protecting against man-in-the-middle attacks. Only consider disabling cephx if your network is private!"]
    #[doc = ""]
    pub disable_cephx: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Minimum number of available replicas per object to allow I/O"]
    #[doc = ""]
    pub min_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Use specific network for all ceph related traffic"]
    #[doc = ""]
    pub network: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Placement group bits, used to specify the default number of placement groups."]
    #[doc = ""]
    #[doc = "Depreacted. This setting was deprecated in recent Ceph versions."]
    #[doc = ""]
    pub pg_bits: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Targeted number of replicas per object"]
    #[doc = ""]
    pub size: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
