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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Status {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
}
impl GetOutput {
    pub fn new(
        id: String,
        node: String,
        pid: u64,
        starttime: f64,
        status: Status,
        ty: String,
        upid: String,
        user: String,
    ) -> Self {
        Self {
            id,
            node,
            pid,
            starttime,
            status,
            ty,
            upid,
            user,
            exitstatus: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub exitstatus: Option<String>,
    pub id: String,
    pub node: String,
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    pub pid: u64,
    #[serde(
        serialize_with = "crate::serialize_number",
        deserialize_with = "crate::deserialize_number"
    )]
    pub starttime: f64,
    pub status: Status,
    #[serde(rename = "type")]
    pub ty: String,
    pub upid: String,
    pub user: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read task status."]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
