pub mod userid;
#[derive(Debug, Clone)]
pub struct UsersClient<T> {
    client: T,
    path: String,
}
impl<T> UsersClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/users"),
        }
    }
}
impl<T> UsersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "User index."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl<T> UsersClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create new user."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        match self.client.post(&path, &params) {
            Ok(o) => Ok(o),
            Err(e) if crate::client::Error::is_empty_data(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
impl GetOutputItems {
    pub fn new(userid: String) -> Self {
        Self {
            userid,
            comment: Default::default(),
            email: Default::default(),
            enable: Default::default(),
            expire: Default::default(),
            firstname: Default::default(),
            groups: Default::default(),
            keys: Default::default(),
            lastname: Default::default(),
            realm_type: Default::default(),
            tfa_locked_until: Default::default(),
            tokens: Default::default(),
            totp_locked: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
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
    #[serde(rename = "realm-type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The type of the users realm"]
    #[doc = ""]
    pub realm_type: Option<String>,
    #[serde(rename = "tfa-locked-until")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Contains a timestamp until when a user is locked out of 2nd factors."]
    #[doc = ""]
    pub tfa_locked_until: Option<i64>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub tokens: Vec<TokensGetOutputItemsTokensItems>,
    #[serde(rename = "totp-locked")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "True if the user is currently locked out of TOTP factors."]
    #[doc = ""]
    pub totp_locked: Option<bool>,
    #[doc = "Full User ID, in the `name@realm` format."]
    #[doc = ""]
    pub userid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Optional filter for enable property."]
    #[doc = ""]
    pub enabled: Option<bool>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Include group and token information."]
    #[doc = ""]
    pub full: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(userid: String) -> Self {
        Self {
            userid,
            comment: Default::default(),
            email: Default::default(),
            enable: Default::default(),
            expire: Default::default(),
            firstname: Default::default(),
            groups: Default::default(),
            keys: Default::default(),
            lastname: Default::default(),
            password: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Initial password."]
    #[doc = ""]
    pub password: Option<String>,
    #[doc = "Full User ID, in the `name@realm` format."]
    #[doc = ""]
    pub userid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl TokensGetOutputItemsTokensItems {
    pub fn new(tokenid: String) -> Self {
        Self {
            tokenid,
            comment: Default::default(),
            expire: Default::default(),
            privsep: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct TokensGetOutputItemsTokensItems {
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
    #[doc = "User-specific token identifier."]
    #[doc = ""]
    pub tokenid: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> UsersClient<T>
where
    T: crate::client::Client,
{
    pub fn userid(&self, userid: &str) -> userid::UseridClient<T> {
        userid::UseridClient::<T>::new(self.client.clone(), &self.path, userid)
    }
}
