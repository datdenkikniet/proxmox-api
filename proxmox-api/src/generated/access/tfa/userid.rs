pub mod id;
#[derive(Debug, Clone)]
pub struct UseridClient<T> {
    client: T,
    path: String,
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, userid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, userid),
        }
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List TFA configurations of users."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Add a TFA entry for a user."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl GetOutputItems {
    pub fn new(created: i64, description: String, id: String, ty: Type) -> Self {
        Self {
            created,
            description,
            id,
            ty,
            enable: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Creation time of this entry as unix epoch."]
    #[doc = ""]
    pub created: i64,
    #[doc = "User chosen description for this entry."]
    #[doc = ""]
    pub description: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Whether this TFA entry is currently enabled."]
    #[doc = ""]
    pub enable: Option<bool>,
    #[doc = "The id used to reference this entry."]
    #[doc = ""]
    pub id: String,
    #[serde(rename = "type")]
    #[doc = "TFA Entry Type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostOutput {
    pub fn new(id: String) -> Self {
        Self {
            id,
            challenge: Default::default(),
            recovery: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "When adding u2f entries, this contains a challenge the user must respond to in order to finish the registration."]
    #[doc = ""]
    pub challenge: Option<String>,
    #[doc = "The id of a newly added TFA entry."]
    #[doc = ""]
    pub id: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "When adding recovery codes, this contains the list of codes to be displayed to the user"]
    #[doc = ""]
    pub recovery: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(ty: Type) -> Self {
        Self {
            ty,
            challenge: Default::default(),
            description: Default::default(),
            password: Default::default(),
            totp: Default::default(),
            value: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "When responding to a u2f challenge: the original challenge string"]
    #[doc = ""]
    pub challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A description to distinguish multiple entries from one another"]
    #[doc = ""]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current password of the user performing the change."]
    #[doc = ""]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A totp URI."]
    #[doc = ""]
    pub totp: Option<String>,
    #[serde(rename = "type")]
    #[doc = "TFA Entry Type."]
    #[doc = ""]
    pub ty: Type,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current value for the provided totp URI, or a Webauthn/U2F challenge response"]
    #[doc = ""]
    pub value: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "TFA Entry Type."]
#[doc = ""]
pub enum Type {
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "u2f")]
    U2f,
    #[serde(rename = "webauthn")]
    Webauthn,
    #[serde(rename = "yubico")]
    Yubico,
}
impl TryFrom<&str> for Type {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "recovery" => Ok(Self::Recovery),
            "totp" => Ok(Self::Totp),
            "u2f" => Ok(Self::U2f),
            "webauthn" => Ok(Self::Webauthn),
            "yubico" => Ok(Self::Yubico),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
