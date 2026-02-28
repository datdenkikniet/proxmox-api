#[derive(Debug, Clone)]
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
    #[doc = ""]
    pub fn get(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> TicketClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create or verify authentication ticket."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
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
impl PostParams {
    pub fn new(password: String, username: UsernameStr) -> Self {
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
    #[doc = ""]
    pub new_format: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "One-time password for Two-factor authentication."]
    #[doc = ""]
    pub otp: Option<String>,
    #[doc = "The secret password. This can also be a valid ticket."]
    #[doc = ""]
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Verify ticket, and check if user have access 'privs' on 'path'"]
    #[doc = ""]
    pub path: Option<PathStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Verify ticket, and check if user have access 'privs' on 'path'"]
    #[doc = ""]
    pub privs: Option<PrivsStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "You can optionally pass the realm using this parameter. Normally the realm is simply added to the username \\\\<username\\\\>@\\\\<relam\\\\>."]
    #[doc = ""]
    pub realm: Option<RealmStr>,
    #[serde(rename = "tfa-challenge")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The signed TFA challenge string the user wants to respond to."]
    #[doc = ""]
    pub tfa_challenge: Option<String>,
    #[doc = "User name"]
    #[doc = ""]
    pub username: UsernameStr,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PathStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for PathStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(64usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 64";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for PathStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for PathStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PathStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PrivsStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for PrivsStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(64usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 64";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for PrivsStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for PrivsStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PrivsStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RealmStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for RealmStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(32usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 32";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for RealmStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for RealmStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for RealmStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UsernameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for UsernameStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(64usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 64";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for UsernameStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for UsernameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for UsernameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
