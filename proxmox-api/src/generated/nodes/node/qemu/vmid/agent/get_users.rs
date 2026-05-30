#[derive(Debug, Clone)]
pub struct GetUsersClient<T> {
    client: T,
    path: String,
}
impl<T> GetUsersClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/get-users"),
        }
    }
}
impl<T> GetUsersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Execute get-users."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/vms/{vmid}\", [\"VM.GuestAgent.Audit\", \"VM.GuestAgent.Unrestricted\"], any)"]
    pub async fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &()).await
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
