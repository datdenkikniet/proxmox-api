#[derive(Debug, Clone)]
pub struct MigrateClient<T> {
    client: T,
    path: String,
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/migrate"),
        }
    }
}
impl<T> MigrateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Request resource migration (online) to another node."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl BlockingResourcesPostOutputBlockingResourcesItems {
    pub fn new(cause: Cause, sid: String) -> Self {
        Self {
            cause,
            sid,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct BlockingResourcesPostOutputBlockingResourcesItems {
    #[doc = "The reason why the HA resource is blocking the migration."]
    #[doc = ""]
    pub cause: Cause,
    #[doc = "The blocking HA resource id"]
    #[doc = ""]
    pub sid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostOutput {
    pub fn new(requested_node: String, sid: String) -> Self {
        Self {
            requested_node,
            sid,
            blocking_resources: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    #[serde(rename = "blocking-resources")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "HA resources, which are blocking the given HA resource from being migrated to the requested target node."]
    #[doc = ""]
    pub blocking_resources: Vec<BlockingResourcesPostOutputBlockingResourcesItems>,
    #[serde(rename = "requested-node")]
    #[doc = "Node, which was requested to be migrated to."]
    #[doc = ""]
    pub requested_node: String,
    #[doc = "HA resource, which is requested to be migrated."]
    #[doc = ""]
    pub sid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(node: String) -> Self {
        Self {
            node,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Target node."]
    #[doc = ""]
    pub node: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The reason why the HA resource is blocking the migration."]
#[doc = ""]
pub enum Cause {
    #[serde(rename = "resource-affinity")]
    ResourceAffinity,
}
impl TryFrom<&str> for Cause {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "resource-affinity" => Ok(Self::ResourceAffinity),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
