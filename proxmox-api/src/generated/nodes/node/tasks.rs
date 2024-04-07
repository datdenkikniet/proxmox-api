pub mod upid;
pub struct TasksClient<T> {
    client: T,
    path: String,
}
impl<T> TasksClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/tasks"),
        }
    }
}
impl<T> TasksClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read task list for one node (finished tasks)."]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(
        id: String,
        node: String,
        pid: u64,
        pstart: u64,
        starttime: u64,
        ty: String,
        upid: String,
        user: String,
    ) -> Self {
        Self {
            id,
            node,
            pid,
            pstart,
            starttime,
            ty,
            upid,
            user,
            endtime: Default::default(),
            status: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub endtime: Option<u64>,
    pub id: String,
    pub node: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pid: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pstart: u64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub starttime: u64,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<String>,
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks with a status of ERROR."]
    pub errors: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list this amount of tasks."]
    pub limit: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks since this UNIX epoch."]
    pub since: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List archived, active or all tasks."]
    pub source: Option<Source>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List tasks beginning from this offset."]
    pub start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of Task States that should be returned."]
    pub statusfilter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks of this type (e.g., vzstart, vzdump)."]
    pub typefilter: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks until this UNIX epoch."]
    pub until: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks from this user."]
    pub userfilter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks for this VM."]
    pub vmid: Option<crate::types::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Source {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "archive")]
    Archive,
}
impl Default for Source {
    fn default() -> Self {
        Self::Archive
    }
}
impl<T> TasksClient<T>
where
    T: crate::client::Client,
{
    pub fn upid(&self, upid: &str) -> upid::UpidClient<T> {
        upid::UpidClient::<T>::new(self.client.clone(), &self.path, upid)
    }
}
