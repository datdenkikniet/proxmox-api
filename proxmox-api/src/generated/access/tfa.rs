pub mod userid;
pub struct TfaClient<T> {
    client: T,
    path: String,
}
impl<T> TfaClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/tfa"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
impl EntriesGetOutputItemsEntriesItems {
    pub fn new(created: u64, description: String, id: String, ty: Type) -> Self {
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
pub struct EntriesGetOutputItemsEntriesItems {
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "Creation time of this entry as unix epoch."]
    pub created: u64,
    #[doc = "User chosen description for this entry."]
    pub description: String,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Whether this TFA entry is currently enabled."]
    pub enable: Option<bool>,
    #[doc = "The id used to reference this entry."]
    pub id: String,
    #[serde(rename = "type")]
    #[doc = "TFA Entry Type."]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutputItems {
    pub fn new(entries: Vec<EntriesGetOutputItemsEntriesItems>, userid: String) -> Self {
        Self {
            entries,
            userid,
            tfa_locked_until: Default::default(),
            totp_locked: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub entries: Vec<EntriesGetOutputItemsEntriesItems>,
    #[serde(rename = "tfa-locked-until")]
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Contains a timestamp until when a user is locked out of 2nd factors."]
    pub tfa_locked_until: Option<u64>,
    #[serde(rename = "totp-locked")]
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "True if the user is currently locked out of TOTP factors."]
    pub totp_locked: Option<bool>,
    #[doc = "User this entry belongs to."]
    pub userid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> TfaClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List TFA configurations of users."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> TfaClient<T>
where
    T: crate::client::Client,
{
    pub fn userid(&self, userid: &str) -> userid::UseridClient<T> {
        userid::UseridClient::<T>::new(self.client.clone(), &self.path, userid)
    }
}
