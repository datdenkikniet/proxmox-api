#[derive(Debug, Clone)]
pub struct ExtractconfigClient<T> {
    client: T,
    path: String,
}
impl<T> ExtractconfigClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/extractconfig"),
        }
    }
}
impl<T> ExtractconfigClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Extract configuration from vzdump backup archive."]
    #[doc = ""]
    #[doc = "The user needs 'VM.Backup' permissions on the backed up guest ID, and 'Datastore.AllocateSpace' on the backup storage."]
    pub async fn get(&self, params: GetParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params).await
    }
}
impl GetParams {
    pub fn new(volume: String) -> Self {
        Self {
            volume,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "Volume identifier"]
    #[doc = ""]
    pub volume: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
