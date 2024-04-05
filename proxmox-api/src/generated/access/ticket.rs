pub struct TicketClient<T> {
    client: T,
    path: String,
}
impl<T> TicketClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ticket"),
        }
    }
}
impl<T> TicketClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Dummy. Useful for formatters which want to provide a login page."]
    pub fn get(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl PostParams {
    pub fn new(password: String, username: String) -> Self {
        Self {
            password,
            username,
            new_format: Default::default(),
            otp: Default::default(),
            path: Default::default(),
            privs: Default::default(),
            realm: Default::default(),
            tfa_challenge: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(rename = "new-format")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "This parameter is now ignored and assumed to be 1."]
    pub new_format: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "One-time password for Two-factor authentication."]
    pub otp: Option<String>,
    #[doc = "The secret password. This can also be a valid ticket."]
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Verify ticket, and check if user have access 'privs' on 'path'"]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Verify ticket, and check if user have access 'privs' on 'path'"]
    pub privs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "You can optionally pass the realm using this parameter. Normally the realm is simply added to the username \\<username\\>@\\<relam\\>."]
    pub realm: Option<String>,
    #[serde(rename = "tfa-challenge")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The signed TFA challenge string the user wants to respond to."]
    pub tfa_challenge: Option<String>,
    #[doc = "User name"]
    pub username: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostOutput {
    pub fn new(username: String) -> Self {
        Self {
            username,
            csrfpreventiontoken: Default::default(),
            clustername: Default::default(),
            ticket: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    #[serde(rename = "CSRFPreventionToken")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub csrfpreventiontoken: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub clustername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub ticket: Option<String>,
    pub username: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> TicketClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create or verify authentication ticket."]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
