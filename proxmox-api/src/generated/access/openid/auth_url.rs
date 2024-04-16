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
impl<'a, T> crate::ProxmoxClient for &'a AuthUrlClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> AuthUrlClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the OpenId Authorization Url for the specified realm."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, String, T::Error>
    for &AuthUrlClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<String, T::Error> {
        self.post(params)
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
