pub struct PermissionsClient<T> {
    client: T,
    path: String,
}
impl<T> PermissionsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/permissions"),
        }
    }
}
impl<T> PermissionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Retrieve effective permissions of given user/token."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Only dump this specific path, not the whole tree."]
    #[doc = ""]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "User ID or full API token ID"]
    #[doc = ""]
    pub userid: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
