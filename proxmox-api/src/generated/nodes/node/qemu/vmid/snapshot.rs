pub mod snapname;
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
    pub description: String,
    #[doc = "Snapshot identifier. Value 'current' identifies the current VM."]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Parent snapshot identifier."]
    pub parent: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Snapshot creation time"]
    pub snaptime: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Snapshot includes RAM."]
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
    #[doc = "List all snapshots."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
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
    pub description: Option<String>,
    #[doc = "The name of the snapshot."]
    pub snapname: String,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Save the vmstate"]
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
    #[doc = "Snapshot a VM."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> SnapshotClient<T>
where
    T: crate::client::Client,
{
    pub fn snapname(&self, snapname: &str) -> snapname::SnapnameClient<T> {
        snapname::SnapnameClient::<T>::new(self.client.clone(), &self.path, snapname)
    }
}
