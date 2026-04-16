#[derive(Debug, Clone)]
pub struct RollbackClient<T> {
    client: T,
    path: String,
}
impl<T> RollbackClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/rollback"),
        }
    }
}
impl<T> RollbackClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Rollback pending changes to SDN configuration"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "lock-token")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "the token for unlocking the global SDN configuration"]
    #[doc = ""]
    pub lock_token: Option<String>,
    #[serde(rename = "release-lock")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "When lock-token has been provided and configuration successfully rollbacked, release the lock automatically afterwards"]
    #[doc = ""]
    pub release_lock: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
