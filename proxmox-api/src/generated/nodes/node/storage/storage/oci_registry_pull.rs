#[derive(Debug, Clone)]
pub struct OciRegistryPullClient<T> {
    client: T,
    path: String,
}
impl<T> OciRegistryPullClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/oci-registry-pull"),
        }
    }
}
impl<T> OciRegistryPullClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Pull an OCI image from a registry."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(reference: ReferenceStr) -> Self {
        Self {
            reference,
            filename: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Custom destination file name of the OCI image. Caution: This will be normalized!"]
    #[doc = ""]
    pub filename: Option<FilenameStr>,
    #[doc = "The reference to the OCI image to download."]
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
pub struct FilenameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for FilenameStr {
    const MIN_LENGTH: Option<usize> = Some(1usize);
    const MAX_LENGTH: Option<usize> = Some(255usize);
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = None::<&'static str>;
    const TYPE_DESCRIPTION: &'static str = "a string with length between 1 and 255";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for FilenameStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for FilenameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::bounded_string::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for FilenameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::bounded_string::deserialize_bounded_string(deserializer)
    }
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
        "^(?:(?:[a-zA-Z\\d]|[a-zA-Z\\d][a-zA-Z\\d-]*[a-zA-Z\\d])(?:\\.(?:[a-zA-Z\\d]|[a-zA-Z\\d][a-zA-Z\\d-]*[a-zA-Z\\d]))*(?::\\d+)?/)?[a-z\\d]+(?:(?:[._]|__|[-]*)[a-z\\d]+)*(?:/[a-z\\d]+(?:(?:[._]|__|[-]*)[a-z\\d]+)*)*:\\w[\\w.-]{0,127}$",
    );
    const TYPE_DESCRIPTION: &'static str = "a string with pattern r\"^(?:(?:[a-zA-Z\\d]|[a-zA-Z\\d][a-zA-Z\\d-]*[a-zA-Z\\d])(?:\\.(?:[a-zA-Z\\d]|[a-zA-Z\\d][a-zA-Z\\d-]*[a-zA-Z\\d]))*(?::\\d+)?/)?[a-z\\d]+(?:(?:[._]|__|[-]*)[a-z\\d]+)*(?:/[a-z\\d]+(?:(?:[._]|__|[-]*)[a-z\\d]+)*)*:\\w[\\w.-]{0,127}$\" and no length constraints";
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
