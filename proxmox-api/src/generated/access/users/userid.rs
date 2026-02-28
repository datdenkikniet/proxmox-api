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
    pub comment: Option<CommentStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email: Option<EmailStr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable the account (default). You can set this to '0' to disable the account"]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Account expiration date (seconds since epoch). '0' means no expiration date."]
    #[doc = ""]
    pub expire: Option<ExpireInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub firstname: Option<FirstnameStr>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    pub groups: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Keys for two factor auth (yubico)."]
    #[doc = ""]
    pub keys: Option<KeysStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lastname: Option<LastnameStr>,
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
    pub comment: Option<CommentStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email: Option<EmailStr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable the account (default). You can set this to '0' to disable the account"]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Account expiration date (seconds since epoch). '0' means no expiration date."]
    #[doc = ""]
    pub expire: Option<ExpireInt>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub firstname: Option<FirstnameStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub groups: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Keys for two factor auth (yubico)."]
    #[doc = ""]
    pub keys: Option<KeysStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub lastname: Option<LastnameStr>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "API token expiration date (seconds since epoch). '0' means no expiration date."]
    #[doc = ""]
    pub expire: Option<ExpireInt>,
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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ExpireInt(i128);
impl crate::types::bounded_integer::BoundedInteger for ExpireInt {
    const MIN: Option<i128> = Some(0i128);
    const MAX: Option<i128> = None::<i128>;
    const DEFAULT: Option<i128> = None::<i128>;
    const TYPE_DESCRIPTION: &'static str = "an integer greater than or equal to 0";
    fn get(&self) -> i128 {
        self.0
    }
    fn new(value: i128) -> Result<Self, crate::types::bounded_integer::BoundedIntegerError> {
        Self::validate(value)?;
        Ok(Self(value))
    }
}
impl std::convert::TryFrom<i128> for ExpireInt {
    type Error = crate::types::bounded_integer::BoundedIntegerError;
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        crate::types::bounded_integer::BoundedInteger::new(value)
    }
}
impl ::serde::Serialize for ExpireInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_integer(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ExpireInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_integer(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CommentStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for CommentStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(2048usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 2048";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for CommentStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for CommentStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for CommentStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct EmailStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for EmailStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(254usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 254";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for EmailStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for EmailStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for EmailStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FirstnameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for FirstnameStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(1024usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 1024";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for FirstnameStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for FirstnameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for FirstnameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct KeysStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for KeysStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("[0-9a-zA-Z!=]{0,4096}");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"[0-9a-zA-Z!=]{0,4096}\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for KeysStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for KeysStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for KeysStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct LastnameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for LastnameStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(1024usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 1024";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for LastnameStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for LastnameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for LastnameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
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
