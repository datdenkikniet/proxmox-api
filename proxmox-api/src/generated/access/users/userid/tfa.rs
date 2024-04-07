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
impl<T> TfaClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get user TFA types (Personal and Realm)."]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The type of TFA the users realm has set, if any."]
    pub realm: Option<Realm>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Array of the user configured TFA types, if any. Only available if 'multiple' was not passed."]
    pub types: Vec<Types>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The type of TFA the user has set, if any. Only set if 'multiple' was not passed."]
    pub user: Option<User>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Request all entries as an array."]
    pub multiple: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Realm {
    #[serde(rename = "oath")]
    Oath,
    #[serde(rename = "yubico")]
    Yubico,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Types {
    #[serde(rename = "recovedry")]
    Recovedry,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "u2f")]
    U2f,
    #[serde(rename = "webauthn")]
    Webauthn,
    #[serde(rename = "yubico")]
    Yubico,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum User {
    #[serde(rename = "oath")]
    Oath,
    #[serde(rename = "u2f")]
    U2f,
}
