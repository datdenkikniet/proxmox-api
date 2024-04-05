pub struct MacrosClient<T> {
    client: T,
    path: String,
}
impl<T> MacrosClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/macros"),
        }
    }
}
impl GetOutputItems {
    pub fn new(descr: String, macro_def: String) -> Self {
        Self {
            descr,
            macro_def,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "More verbose description (if available)."]
    pub descr: String,
    #[serde(rename = "macro")]
    #[doc = "Macro name."]
    pub macro_def: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MacrosClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List available macros"]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
