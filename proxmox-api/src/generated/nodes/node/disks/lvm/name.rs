pub struct NameClient<T> {
    client: T,
    path: String,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(rename = "cleanup-config")]
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Marks associated storage(s) as not available on this node anymore or removes them from the configuration (if configured for this node only)."]
    pub cleanup_config: Option<bool>,
    #[serde(rename = "cleanup-disks")]
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Also wipe disks so they can be repurposed afterwards."]
    pub cleanup_disks: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Remove an LVM Volume Group."]
    pub fn delete(&self, params: DeleteParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
