pub struct DbClient<T> {
    client: T,
    path: String,
}
impl<T> DbClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/db"),
        }
    }
}
impl GetOutputItems {
    pub fn new(
        can_update_at_runtime: bool,
        level: String,
        mask: String,
        name: String,
        section: String,
        value: String,
    ) -> Self {
        Self {
            can_update_at_runtime,
            level,
            mask,
            name,
            section,
            value,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    pub can_update_at_runtime: bool,
    pub level: String,
    pub mask: String,
    pub name: String,
    pub section: String,
    pub value: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DbClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the Ceph configuration database."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
