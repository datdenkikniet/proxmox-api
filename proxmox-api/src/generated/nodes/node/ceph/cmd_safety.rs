pub struct CmdSafetyClient<T> {
    client: T,
    path: String,
}
impl<T> CmdSafetyClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/cmd-safety"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Action {
    #[serde(rename = "destroy")]
    Destroy,
    #[serde(rename = "stop")]
    Stop,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Service {
    #[serde(rename = "mds")]
    Mds,
    #[serde(rename = "mon")]
    Mon,
    #[serde(rename = "osd")]
    Osd,
}
impl GetParams {
    pub fn new(action: Action, id: String, service: Service) -> Self {
        Self {
            action,
            id,
            service,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "Action to check"]
    pub action: Action,
    #[doc = "ID of the service"]
    pub id: String,
    #[doc = "Service type"]
    pub service: Service,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
    pub fn new(safe: bool) -> Self {
        Self {
            safe,
            status: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(
        serialize_with = "crate::serialize_bool",
        deserialize_with = "crate::deserialize_bool"
    )]
    #[doc = "If it is safe to run the command."]
    pub safe: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Status message given by Ceph."]
    pub status: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> CmdSafetyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Heuristical check if it is safe to perform an action."]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
