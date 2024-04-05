pub struct ChangelogClient<T> {
    client: T,
    path: String,
}
impl<T> ChangelogClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/changelog"),
        }
    }
}
impl GetParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "Package name."]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Package version."]
    pub version: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ChangelogClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get package changelogs."]
    pub fn get(&self, params: GetParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
