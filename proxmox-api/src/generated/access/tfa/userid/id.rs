#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Delete a TFA entry by ID."]
    #[doc = ""]
    pub fn delete(&self, params: DeleteParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Fetch a requested TFA entry if present."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Add a TFA entry for a user."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current password of the user performing the change."]
    #[doc = ""]
    pub password: Option<PasswordStr>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetOutput {
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
pub struct GetOutput {
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A description to distinguish multiple entries from one another"]
    #[doc = ""]
    pub description: Option<DescriptionStr>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Whether the entry should be enabled for login."]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The current password of the user performing the change."]
    #[doc = ""]
    pub password: Option<PasswordStr>,
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
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DescriptionStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DescriptionStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = Some(255usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length at most 255";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DescriptionStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DescriptionStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DescriptionStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PasswordStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for PasswordStr {
    const MIN_LENGTH: Option<usize> = Some(5usize);
    const MAX_LENGTH: Option<usize> = Some(64usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length between 5 and 64";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for PasswordStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for PasswordStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for PasswordStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
