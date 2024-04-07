pub struct TosClient<T> {
    client: T,
    path: String,
}
impl<T> TosClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/tos"),
        }
    }
}
impl<T> TosClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Retrieve ACME TermsOfService URL from CA. Deprecated, please use /cluster/acme/meta."]
    pub fn get(&self, params: GetParams) -> Result<Option<String>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL of ACME CA directory endpoint."]
    pub directory: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
