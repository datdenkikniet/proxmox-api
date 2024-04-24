pub mod snapname;
#[derive(Debug, Clone)]
pub struct SnapshotClient<T> {
    client: T,
    path: String,
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/snapshot"),
        }
    }
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List all snapshots."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Snapshot a VM."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(description: String, name: String) -> Self {
        Self {
            description,
            name,
            parent: Default::default(),
            snaptime: Default::default(),
            vmstate: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Snapshot description."]
    #[doc = ""]
    pub description: String,
    #[doc = "Snapshot identifier. Value 'current' identifies the current VM."]
    #[doc = ""]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Parent snapshot identifier."]
    #[doc = ""]
    pub parent: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Snapshot creation time"]
    #[doc = ""]
    pub snaptime: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Snapshot includes RAM."]
    #[doc = ""]
    pub vmstate: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(snapname: String) -> Self {
        Self {
            snapname,
            description: Default::default(),
            vmstate: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A textual description or comment."]
    #[doc = ""]
    pub description: Option<String>,
    #[doc = "The name of the snapshot."]
    #[doc = ""]
    pub snapname: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Save the vmstate"]
    #[doc = ""]
    pub vmstate: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    pub fn snapname(&self, snapname: &str) -> snapname::SnapnameClient<T> {
        snapname::SnapnameClient::<T>::new(self.client.clone(), &self.path, snapname)
    }
}
