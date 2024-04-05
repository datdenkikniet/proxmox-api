pub mod tokenid;
pub struct TokenClient<T> {
    client: T,
    path: String,
}
impl<T> TokenClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/token"),
        }
    }
}
impl GetOutputItems {
    pub fn new(tokenid: String) -> Self {
        Self {
            tokenid,
            comment: Default::default(),
            expire: Default::default(),
            privsep: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "API token expiration date (seconds since epoch). '0' means no expiration date."]
    pub expire: Option<()>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user."]
    pub privsep: Option<bool>,
    #[doc = "User-specific token identifier."]
    pub tokenid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> TokenClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get user API tokens."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> TokenClient<T>
where
    T: crate::client::Client,
{
    pub fn tokenid(&self, tokenid: &str) -> tokenid::TokenidClient<T> {
        tokenid::TokenidClient::<T>::new(self.client.clone(), &self.path, tokenid)
    }
}
