#[derive(Debug, Clone)]
pub struct JournalClient<T> {
    client: T,
    path: String,
}
impl<T> JournalClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/journal"),
        }
    }
}
impl<T> JournalClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read Journal"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<String>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "End before the given Cursor. Conflicts with 'until'"]
    #[doc = ""]
    pub endcursor: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit to the last X lines. Conflicts with a range."]
    #[doc = ""]
    pub lastentries: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display all log since this UNIX epoch. Conflicts with 'startcursor'."]
    #[doc = ""]
    pub since: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Start after the given Cursor. Conflicts with 'since'"]
    #[doc = ""]
    pub startcursor: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display all log until this UNIX epoch. Conflicts with 'endcursor'."]
    #[doc = ""]
    pub until: Option<i64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
