pub struct JoinClient<T> {
    client: T,
    path: String,
}
impl<T> JoinClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/join"),
        }
    }
}
impl<T> JoinClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get information needed to join this cluster over the connected node."]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> JoinClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Joins this node into an existing cluster. If no links are given, default to IP resolved by node's hostname on single link (fallback fails for clusters with multiple links)."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutput {
    pub fn new(
        config_digest: String,
        nodelist: Vec<NodelistGetOutputNodelistItems>,
        preferred_node: String,
        totem: TotemGetOutputTotem,
    ) -> Self {
        Self {
            config_digest,
            nodelist,
            preferred_node,
            totem,
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    pub config_digest: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub nodelist: Vec<NodelistGetOutputNodelistItems>,
    #[doc = "The cluster node name."]
    pub preferred_node: String,
    pub totem: TotemGetOutputTotem,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The node for which the joinee gets the nodeinfo. "]
    pub node: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl NodelistGetOutputNodelistItems {
    pub fn new(
        name: String,
        pve_addr: ::std::net::IpAddr,
        pve_fp: String,
        quorum_votes: u64,
    ) -> Self {
        Self {
            name,
            pve_addr,
            pve_fp,
            quorum_votes,
            nodeid: Default::default(),
            ring0_addr: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct NodelistGetOutputNodelistItems {
    #[doc = "The cluster node name."]
    pub name: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node id for this node."]
    pub nodeid: Option<u64>,
    pub pve_addr: ::std::net::IpAddr,
    #[doc = "Certificate SHA 256 fingerprint."]
    pub pve_fp: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub quorum_votes: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    pub ring0_addr: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(fingerprint: String, hostname: String, password: String) -> Self {
        Self {
            fingerprint,
            hostname,
            password,
            force: Default::default(),
            links: Default::default(),
            nodeid: Default::default(),
            votes: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Certificate SHA 256 fingerprint."]
    pub fingerprint: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Do not throw error if node already exists."]
    pub force: Option<bool>,
    #[doc = "Hostname (or IP) of an existing cluster member."]
    pub hostname: String,
    #[serde(rename = "link[n]")]
    #[serde(
        serialize_with = "crate::types::serialize_multi::<NumberedLinks, _>",
        deserialize_with = "crate::types::deserialize_multi::<NumberedLinks, _>"
    )]
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty", default)]
    #[serde(flatten)]
    #[doc = "Address and priority information of a single corosync link. (up to 8 links supported; link0..link7)"]
    pub links: ::std::collections::HashMap<u32, String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Node id for this node."]
    pub nodeid: Option<u64>,
    #[doc = "Superuser (root) password of peer node."]
    pub password: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Number of votes for this node"]
    pub votes: Option<u64>,
    #[serde(
        flatten,
        deserialize_with = "crate::types::multi::deserialize_additional_data::<'_, PostParams, _, _>"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl crate::types::multi::Test for PostParams {
    fn test_fn() -> fn(&str) -> bool {
        fn the_test(input: &str) -> bool {
            let array = [
                <NumberedLinks as crate::types::multi::NumberedItems>::key_matches
                    as fn(&str) -> bool,
            ];
            array.iter().any(|f| f(input))
        }
        the_test as _
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct TotemGetOutputTotem {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Default)]
struct NumberedLinks;
impl crate::types::multi::NumberedItems for NumberedLinks {
    type Item = String;
    const PREFIX: &'static str = "link";
}
