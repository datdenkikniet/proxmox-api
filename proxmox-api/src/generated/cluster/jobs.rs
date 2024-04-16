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
impl<'a, T> crate::ProxmoxClient for &'a JobsClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> JobsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index for jobs related endpoints."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &JobsClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
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
    #[doc = ""]
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
