#[derive(Debug, Clone)]
pub struct QueryOciRepoTagsClient<T> {
    client: T,
    path: String,
}
impl<T> QueryOciRepoTagsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/query-oci-repo-tags"),
        }
    }
}
impl<T> QueryOciRepoTagsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List all tags for an OCI repository reference."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<Vec<String>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
impl GetParams {
    pub fn new(reference: ReferenceStr) -> Self {
        Self {
            reference,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "The reference to the repository to query tags from."]
    #[doc = ""]
    pub reference: ReferenceStr,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ReferenceStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for ReferenceStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some(
        "^(?:(?:[a-zA-Z\\d]|[a-zA-Z\\d][a-zA-Z\\d-]*[a-zA-Z\\d])(?:\\.(?:[a-zA-Z\\d]|[a-zA-Z\\d][a-zA-Z\\d-]*[a-zA-Z\\d]))*(?::\\d+)?/)?[a-z\\d]+(?:(?:[._]|__|[-]*)[a-z\\d]+)*(?:/[a-z\\d]+(?:(?:[._]|__|[-]*)[a-z\\d]+)*)*$",
    );
    const TYPE_DESCRIPTION: &'static str = "a string with pattern r\"^(?:(?:[a-zA-Z\\d]|[a-zA-Z\\d][a-zA-Z\\d-]*[a-zA-Z\\d])(?:\\.(?:[a-zA-Z\\d]|[a-zA-Z\\d][a-zA-Z\\d-]*[a-zA-Z\\d]))*(?::\\d+)?/)?[a-z\\d]+(?:(?:[._]|__|[-]*)[a-z\\d]+)*(?:/[a-z\\d]+(?:(?:[._]|__|[-]*)[a-z\\d]+)*)*$\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for ReferenceStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for ReferenceStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ReferenceStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
}
