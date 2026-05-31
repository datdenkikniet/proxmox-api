#[derive(Debug, Clone)]
pub struct SuspendallClient<T> {
    client: T,
    path: String,
}
impl<T> SuspendallClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/suspendall"),
        }
    }
}
impl<T> SuspendallClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Suspend all VMs."]
    #[doc = ""]
    #[doc = "The 'VM.PowerMgmt' permission is required on '/' or on '/vms/\\<ID\\>' for each ID passed via the 'vms' parameter. Additionally, you need 'VM.Config.Disk' on the '/vms/{vmid}' path and 'Datastore.AllocateSpace' for the configured state-storage(s)"]
    pub async fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params).await
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only consider Guests with these IDs."]
    #[doc = ""]
    pub vms: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
