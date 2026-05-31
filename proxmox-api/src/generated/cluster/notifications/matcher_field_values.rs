#[derive(Debug, Clone)]
pub struct MatcherFieldValuesClient<T> {
    client: T,
    path: String,
}
impl<T> MatcherFieldValuesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/matcher-field-values"),
        }
    }
}
impl<T> MatcherFieldValuesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Returns known notification metadata fields and their known values"]
    #[doc = ""]
    #[doc = "Permission check: or(perm(\"/mapping/notifications\", [\"Mapping.Modify\"]), perm(\"/mapping/notifications\", [\"Mapping.Audit\"]))"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(field: String, value: String) -> Self {
        Self {
            field,
            value,
            comment: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Additional comment for this value."]
    #[doc = ""]
    pub comment: Option<String>,
    #[doc = "Field this value belongs to."]
    #[doc = ""]
    pub field: String,
    #[doc = "Notification metadata value known by the system."]
    #[doc = ""]
    pub value: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
