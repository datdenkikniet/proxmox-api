#[derive(Debug, Clone)]
pub struct AuthUrlClient<T> {
    client: T,
    path: String,
}
impl<T> AuthUrlClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/auth-url"),
        }
    }
}
impl<T> AuthUrlClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the OpenId Authorization Url for the specified realm."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(realm: String, redirect_url: String) -> Self {
        Self {
            realm,
            redirect_url,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Authentication domain ID"]
    #[doc = ""]
    pub realm: String,
    #[serde(rename = "redirect-url")]
    #[doc = "Redirection Url. The client should set this to the used server url (location.origin)."]
    #[doc = ""]
    pub redirect_url: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
