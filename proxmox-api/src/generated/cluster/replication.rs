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
impl<'a, T> crate::ProxmoxClient for &'a ReplicationClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ReplicationClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List replication jobs."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> ReplicationClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a new replication job"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
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
    #[doc = ""]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Flag to disable/deactivate the entry."]
    #[doc = ""]
    pub disable: Option<bool>,
    #[doc = "Replication Job ID. The ID is composed of a Guest ID and a job number, separated by a hyphen, i.e. '\\\\<GUEST\\\\>-\\\\<JOBNUM\\\\>'."]
    #[doc = ""]
    pub id: String,
    #[serde(
        serialize_with = "crate::types::serialize_number_optional",
        deserialize_with = "crate::types::deserialize_number_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Rate limit in mbps (megabytes per second) as floating point number."]
    #[doc = ""]
    pub rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Mark the replication job for removal. The job will remove all local replication snapshots. When set to 'full', it also tries to remove replicated volumes on the target. The job then removes itself from the configuration file."]
    #[doc = ""]
    pub remove_job: Option<RemoveJob>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Storage replication schedule. The format is a subset of `systemd` calendar events."]
    #[doc = ""]
    pub schedule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "For internal use, to detect if the guest was stolen."]
    #[doc = ""]
    pub source: Option<String>,
    #[doc = "Target node."]
    #[doc = ""]
    pub target: String,
    #[serde(rename = "type")]
    #[doc = "Section type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Mark the replication job for removal. The job will remove all local replication snapshots. When set to 'full', it also tries to remove replicated volumes on the target. The job then removes itself from the configuration file."]
#[doc = ""]
pub enum RemoveJob {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "local")]
    Local,
}
impl TryFrom<&str> for RemoveJob {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "full" => Ok(Self::Full),
            "local" => Ok(Self::Local),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
#[doc = "Section type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "local")]
    Local,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "local" => Ok(Self::Local),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> ReplicationClient<T>
where
    T: crate::client::Client,
{
    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
