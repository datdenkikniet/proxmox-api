#[derive(Debug, Clone)]
pub struct MetaClient<T> {
    client: T,
    path: String,
}
impl<T> MetaClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/meta"),
        }
    }
}
impl<T> MetaClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Retrieve ACME Directory Meta Information"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(rename = "caaIdentities")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Hostnames referring to the ACME servers."]
    #[doc = ""]
    pub caaidentities: Vec<String>,
    #[serde(rename = "externalAccountRequired")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "EAB Required"]
    #[doc = ""]
    pub externalaccountrequired: Option<bool>,
    #[serde(rename = "termsOfService")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ACME TermsOfService URL."]
    #[doc = ""]
    pub termsofservice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL to more information about the ACME server."]
    #[doc = ""]
    pub website: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL of ACME CA directory endpoint."]
    #[doc = ""]
    pub directory: Option<DirectoryStr>,
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
