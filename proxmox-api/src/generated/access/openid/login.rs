#[derive(Debug, Clone)]
pub struct LoginClient<T> {
    client: T,
    path: String,
}
impl<T> LoginClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/login"),
        }
    }
}
impl<T> LoginClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Verify OpenID authorization code and create a ticket."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(code: String, redirect_url: String, state: String) -> Self {
        Self {
            code,
            redirect_url,
            state,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "OpenId authorization code."]
    #[doc = ""]
    pub code: String,
    #[serde(rename = "redirect-url")]
    #[doc = "Redirection Url. The client should set this to the used server url (location.origin)."]
    #[doc = ""]
    pub redirect_url: String,
    #[doc = "OpenId state."]
    #[doc = ""]
    pub state: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
