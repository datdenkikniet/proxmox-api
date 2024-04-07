pub mod realm_sync;
pub mod schedule_analyze;
pub struct JobsClient<T> {
    client: T,
    path: String,
}
impl<T> JobsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/jobs"),
        }
    }
}
impl<T> JobsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index for jobs related endpoints."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(subdir: String) -> Self {
        Self {
            subdir,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "API sub-directory endpoint"]
    pub subdir: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> JobsClient<T>
where
    T: crate::client::Client,
{
    pub fn realm_sync(&self) -> realm_sync::RealmSyncClient<T> {
        realm_sync::RealmSyncClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> JobsClient<T>
where
    T: crate::client::Client,
{
    pub fn schedule_analyze(&self) -> schedule_analyze::ScheduleAnalyzeClient<T> {
        schedule_analyze::ScheduleAnalyzeClient::<T>::new(self.client.clone(), &self.path)
    }
}
