pub mod upid;
#[derive(Debug, Clone)]
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
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(
        id: String,
        node: String,
        pid: i64,
        pstart: i64,
        starttime: i64,
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
    pub endtime: Option<i64>,
    pub id: String,
    pub node: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pid: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub pstart: i64,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    pub starttime: i64,
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
    #[doc = ""]
    pub errors: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list this amount of tasks."]
    #[doc = ""]
    pub limit: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks since this UNIX epoch."]
    #[doc = ""]
    pub since: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List archived, active or all tasks."]
    #[doc = ""]
    pub source: Option<Source>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List tasks beginning from this offset."]
    #[doc = ""]
    pub start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of Task States that should be returned."]
    #[doc = ""]
    pub statusfilter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks of this type (e.g., vzstart, vzdump)."]
    #[doc = ""]
    pub typefilter: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks until this UNIX epoch."]
    #[doc = ""]
    pub until: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks from this user."]
    #[doc = ""]
    pub userfilter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only list tasks for this VM."]
    #[doc = ""]
    pub vmid: Option<crate::types::VmId>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "List archived, active or all tasks."]
#[doc = ""]
pub enum Source {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "archive")]
    Archive,
}
impl TryFrom<&str> for Source {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "active" => Ok(Self::Active),
            "all" => Ok(Self::All),
            "archive" => Ok(Self::Archive),
            v => Err(format!("Unknown variant {v}")),
        }
    }
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
