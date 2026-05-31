#[derive(Debug, Clone)]
pub struct MtunnelClient<T> {
    client: T,
    path: String,
}
impl<T> MtunnelClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/mtunnel"),
        }
    }
}
impl<T> MtunnelClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migration tunnel endpoint - only for internal use by CT migration."]
    #[doc = ""]
    #[doc = "Permission check: and(perm(\"/vms/{vmid}\", [\"VM.Allocate\"]), perm(\"/\", [\"Sys.Incoming\"]))"]
    #[doc = "You need 'VM.Allocate' permissions on '/vms/{vmid}' and Sys.Incoming on '/'. Further permission checks happen during the actual migration."]
    pub async fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params).await
    }
}
impl PostOutput {
    pub fn new(socket: String, ticket: String, upid: String) -> Self {
        Self {
            socket,
            ticket,
            upid,
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    pub socket: String,
    pub ticket: String,
    pub upid: String,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of network bridges to check availability. Will be checked again for actually used bridges during migration."]
    #[doc = ""]
    pub bridges: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "List of storages to check permission and availability. Will be checked again for all actually used storages during migration."]
    #[doc = ""]
    pub storages: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
