pub mod id;
pub struct ReplicationClient<T> {
    client: T,
    path: String,
}
impl<T> ReplicationClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/replication"),
        }
    }
}
impl<T> ReplicationClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List replication jobs."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> ReplicationClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new replication job"]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(id: String, target: String, ty: Type) -> Self {
        Self {
            id,
            target,
            ty,
            comment: Default::default(),
            disable: Default::default(),
            rate: Default::default(),
            remove_job: Default::default(),
            schedule: Default::default(),
            source: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Description."]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Flag to disable/deactivate the entry."]
    pub disable: Option<bool>,
    #[doc = "Replication Job ID. The ID is composed of a Guest ID and a job number, separated by a hyphen, i.e. '\\<GUEST\\>-\\<JOBNUM\\>'."]
    pub id: String,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Rate limit in mbps (megabytes per second) as floating point number."]
    pub rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mark the replication job for removal. The job will remove all local replication snapshots. When set to 'full', it also tries to remove replicated volumes on the target. The job then removes itself from the configuration file."]
    pub remove_job: Option<RemoveJob>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Storage replication schedule. The format is a subset of `systemd` calendar events."]
    pub schedule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For internal use, to detect if the guest was stolen."]
    pub source: Option<String>,
    #[doc = "Target node."]
    pub target: String,
    #[serde(rename = "type")]
    #[doc = "Section type."]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum RemoveJob {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "local")]
    Local,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "local")]
    Local,
}
impl<T> ReplicationClient<T>
where
    T: crate::client::Client,
{
    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
