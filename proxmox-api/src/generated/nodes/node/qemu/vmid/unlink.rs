pub struct UnlinkClient<T> {
    client: T,
    path: String,
}
impl<T> UnlinkClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/unlink"),
        }
    }
}
impl<T> UnlinkClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Unlink/delete disk images."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
impl PutParams {
    pub fn new(idlist: String) -> Self {
        Self {
            idlist,
            force: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force physical removal. Without this, we simple remove the disk from the config file and create an additional configuration entry called 'unused\\\\[n\\\\]', which contains the volume ID. Unlink of unused\\\\[n\\\\] always cause physical removal."]
    #[doc = ""]
    pub force: Option<bool>,
    #[doc = "A list of disk IDs you want to delete."]
    #[doc = ""]
    pub idlist: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
