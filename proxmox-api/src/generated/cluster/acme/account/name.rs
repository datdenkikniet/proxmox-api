#[derive(Debug, Clone)]
pub struct NameClient<T> {
    client: T,
    path: String,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Deactivate existing ACME account at CA."]
    #[doc = ""]
    pub fn delete(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Return existing ACME account information."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Update existing ACME account information with CA. Note: not specifying any new account information triggers a refresh."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct AccountGetOutputAccount {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub account: Option<AccountGetOutputAccount>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL of ACME CA directory endpoint."]
    #[doc = ""]
    pub directory: Option<DirectoryStr>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tos: Option<String>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Contact email addresses."]
    #[doc = ""]
    pub contact: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DirectoryStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for DirectoryStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("^https?://.*");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"^https?://.*\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for DirectoryStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for DirectoryStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for DirectoryStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
