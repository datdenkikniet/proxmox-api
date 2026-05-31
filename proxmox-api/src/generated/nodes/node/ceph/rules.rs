#[derive(Debug, Clone)]
pub struct RulesClient<T> {
    client: T,
    path: String,
}
impl<T> RulesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/rules"),
        }
    }
}
impl<T> RulesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List ceph rules."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/\", [\"Sys.Audit\", \"Datastore.Audit\"], any)"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(name: String) -> Self {
        Self {
            name,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Name of the CRUSH rule."]
    #[doc = ""]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
