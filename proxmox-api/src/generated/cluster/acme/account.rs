pub mod name;
#[derive(Debug, Clone)]
pub struct AccountClient<T> {
    client: T,
    path: String,
}
impl<T> AccountClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/account"),
        }
    }
}
impl<T> AccountClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ACMEAccount index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> AccountClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Register a new ACME account with CA."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(contact: String) -> Self {
        Self {
            contact,
            directory: Default::default(),
            eab_hmac_key: Default::default(),
            eab_kid: Default::default(),
            name: Default::default(),
            tos_url: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Contact email addresses."]
    #[doc = ""]
    pub contact: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL of ACME CA directory endpoint."]
    #[doc = ""]
    pub directory: Option<DirectoryStr>,
    #[serde(rename = "eab-hmac-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HMAC key for External Account Binding."]
    #[doc = ""]
    pub eab_hmac_key: Option<String>,
    #[serde(rename = "eab-kid")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Key Identifier for External Account Binding."]
    #[doc = ""]
    pub eab_kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ACME account config file name."]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL of CA TermsOfService - setting this indicates agreement."]
    #[doc = ""]
    pub tos_url: Option<String>,
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
    const DEFAULT: Option<&'static str> = Some("https://acme-v02.api.letsencrypt.org/directory");
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
impl<T> AccountClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
