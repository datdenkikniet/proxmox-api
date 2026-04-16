#[derive(Debug, Clone)]
pub struct CurrentClient<T> {
    client: T,
    path: String,
}
impl<T> CurrentClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/current"),
        }
    }
}
impl<T> CurrentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get HA manger status."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(id: String, node: String, status: String) -> Self {
        Self {
            id,
            node,
            status,
            crm_state: ::std::default::Default::default(),
            failback: ::std::default::Default::default(),
            max_relocate: ::std::default::Default::default(),
            max_restart: ::std::default::Default::default(),
            quorate: ::std::default::Default::default(),
            request_state: ::std::default::Default::default(),
            sid: ::std::default::Default::default(),
            state: ::std::default::Default::default(),
            timestamp: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'service'. Service state as seen by the CRM."]
    #[doc = ""]
    pub crm_state: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The HA resource is automatically migrated to the node with the highest priority according to their node affinity rule, if a node with a higher priority than the current node comes online."]
    #[doc = ""]
    pub failback: Option<bool>,
    #[doc = "Status entry ID (quorum, master, lrm:\\\\<node\\\\>, service:\\\\<sid\\\\>)."]
    #[doc = ""]
    pub id: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'service'."]
    #[doc = ""]
    pub max_relocate: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'service'."]
    #[doc = ""]
    pub max_restart: Option<i64>,
    #[doc = "Node associated to status entry."]
    #[doc = ""]
    pub node: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'quorum'. Whether the cluster is quorate or not."]
    #[doc = ""]
    pub quorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'service'. Requested service state."]
    #[doc = ""]
    pub request_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'service'. Service ID."]
    #[doc = ""]
    pub sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'service'. Verbose service state."]
    #[doc = ""]
    pub state: Option<String>,
    #[doc = "Status of the entry (value depends on type)."]
    #[doc = ""]
    pub status: String,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'lrm','master'. Timestamp of the status information."]
    #[doc = ""]
    pub timestamp: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
