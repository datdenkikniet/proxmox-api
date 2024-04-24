pub mod tfa;
pub mod token;
pub mod unlock_tfa;
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
    #[doc = "Delete user."]
    #[doc = ""]
    pub fn delete(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get user configuration."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update user configuration."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable the account (default). You can set this to '0' to disable the account"]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Account expiration date (seconds since epoch). '0' means no expiration date."]
    #[doc = ""]
    pub expire: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub groups: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Keys for two factor auth (yubico)."]
    #[doc = ""]
    pub keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tokens: Option<TokensGetOutputTokens>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub append: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable the account (default). You can set this to '0' to disable the account"]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Account expiration date (seconds since epoch). '0' means no expiration date."]
    #[doc = ""]
    pub expire: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub groups: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Keys for two factor auth (yubico)."]
    #[doc = ""]
    pub keys: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lastname: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct AdditionalPropertiesGetOutputTokens {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub comment: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "API token expiration date (seconds since epoch). '0' means no expiration date."]
    #[doc = ""]
    pub expire: Option<i64>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Restrict API token privileges with separate ACLs (default), or give full privileges of corresponding user."]
    #[doc = ""]
    pub privsep: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct TokensGetOutputTokens {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties:
        ::std::collections::HashMap<String, AdditionalPropertiesGetOutputTokens>,
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    pub fn tfa(&self) -> tfa::TfaClient<T> {
        tfa::TfaClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    pub fn unlock_tfa(&self) -> unlock_tfa::UnlockTfaClient<T> {
        unlock_tfa::UnlockTfaClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> UseridClient<T>
where
    T: crate::client::Client,
{
    pub fn token(&self) -> token::TokenClient<T> {
        token::TokenClient::<T>::new(self.client.clone(), &self.path)
    }
}
