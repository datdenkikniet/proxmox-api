pub struct SetUserPasswordClient<T> {
    client: T,
    path: String,
}
impl<T> SetUserPasswordClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/set-user-password"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a SetUserPasswordClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> SetUserPasswordClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Sets the password for the given user to the given password"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(password: String, username: String) -> Self {
        Self {
            password,
            username,
            crypted: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "set to 1 if the password has already been passed through crypt()"]
    #[doc = ""]
    pub crypted: Option<bool>,
    #[doc = "The new password."]
    #[doc = ""]
    pub password: String,
    #[doc = "The user to set the password for."]
    #[doc = ""]
    pub username: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
