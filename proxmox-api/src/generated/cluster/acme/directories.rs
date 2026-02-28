#[derive(Debug, Clone)]
pub struct DirectoriesClient<T> {
    client: T,
    path: String,
}
impl<T> DirectoriesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/directories"),
        }
    }
}
impl<T> DirectoriesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get named known ACME directory endpoints."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(name: String, url: UrlStr) -> Self {
        Self { name, url }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub name: String,
    #[doc = "URL of ACME CA directory endpoint."]
    #[doc = ""]
    pub url: UrlStr,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UrlStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for UrlStr {
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
impl std::convert::TryFrom<String> for UrlStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for UrlStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for UrlStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
