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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "End before the given Cursor. Conflicts with 'until'"]
    pub endcursor: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Limit to the last X lines. Conflicts with a range."]
    pub lastentries: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display all log since this UNIX epoch. Conflicts with 'startcursor'."]
    pub since: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Start after the given Cursor. Conflicts with 'since'"]
    pub startcursor: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Display all log until this UNIX epoch. Conflicts with 'endcursor'."]
    pub until: Option<u64>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> JournalClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read Journal"]
    pub fn get(&self, params: GetParams) -> Result<Vec<String>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
