#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/status"),
        }
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get cluster status information."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(id: String, name: String, ty: Type) -> Self {
        Self {
            id,
            name,
            ty,
            ip: Default::default(),
            level: Default::default(),
            local: Default::default(),
            nodeid: Default::default(),
            nodes: Default::default(),
            online: Default::default(),
            quorate: Default::default(),
            version: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "\\\\[node\\\\] IP of the resolved nodename."]
    #[doc = ""]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "\\\\[node\\\\] Proxmox VE Subscription level, indicates if eligible for enterprise support as well as access to the stable Proxmox VE Enterprise Repository."]
    #[doc = ""]
    pub level: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "\\\\[node\\\\] Indicates if this is the responding node."]
    #[doc = ""]
    pub local: Option<bool>,
    pub name: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "\\\\[node\\\\] ID of the node from the corosync configuration."]
    #[doc = ""]
    pub nodeid: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "\\\\[cluster\\\\] Nodes count, including offline nodes."]
    #[doc = ""]
    pub nodes: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "\\\\[node\\\\] Indicates if the node is online or offline."]
    #[doc = ""]
    pub online: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "\\\\[cluster\\\\] Indicates if there is a majority of nodes online to make decisions"]
    #[doc = ""]
    pub quorate: Option<bool>,
    #[serde(rename = "type")]
    #[doc = "Indicates the type, either cluster or node. The type defines the object properties e.g. quorate available for type cluster."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "\\\\[cluster\\\\] Current version of the corosync configuration file."]
    #[doc = ""]
    pub version: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Indicates the type, either cluster or node. The type defines the object properties e.g. quorate available for type cluster."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "node")]
    Node,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "cluster" => Ok(Self::Cluster),
            "node" => Ok(Self::Node),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
