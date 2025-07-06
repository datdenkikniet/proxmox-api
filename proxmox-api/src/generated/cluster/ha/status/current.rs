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
            crm_state: Default::default(),
            max_relocate: Default::default(),
            max_restart: Default::default(),
            quorate: Default::default(),
            request_state: Default::default(),
            sid: Default::default(),
            state: Default::default(),
            timestamp: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For type 'service'. Service state as seen by the CRM."]
    #[doc = ""]
    pub crm_state: Option<String>,
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
