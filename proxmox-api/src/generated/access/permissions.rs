#[derive(Debug, Clone)]
pub struct PermissionsClient<T> {
    client: T,
    path: String,
}
impl<T> PermissionsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/permissions"),
        }
    }
}
impl<T> PermissionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Retrieve effective permissions of given user/token."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
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
    #[doc = "Only dump this specific path, not the whole tree."]
    #[doc = ""]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "User ID or full API token ID"]
    #[doc = ""]
    pub userid: Option<UseridStr>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UseridStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for UseridStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some(
        "(?^:^(?^:[^\\s:/]+)\\@(?^:[A-Za-z][A-Za-z0-9\\.\\-_]+)(?:!(?^:[A-Za-z][A-Za-z0-9\\.\\-_]+))?$)",
    );
    const TYPE_DESCRIPTION: &'static str = "a string with pattern r\"(?^:^(?^:[^\\s:/]+)\\@(?^:[A-Za-z][A-Za-z0-9\\.\\-_]+)(?:!(?^:[A-Za-z][A-Za-z0-9\\.\\-_]+))?$)\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for UseridStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for UseridStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for UseridStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
