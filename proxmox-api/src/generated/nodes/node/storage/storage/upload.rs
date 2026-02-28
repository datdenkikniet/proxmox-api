#[derive(Debug, Clone)]
pub struct UploadClient<T> {
    client: T,
    path: String,
}
impl<T> UploadClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/upload"),
        }
    }
}
impl<T> UploadClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Upload templates and ISO images."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(content: Content, filename: FilenameStr) -> Self {
        Self {
            content,
            filename,
            checksum: Default::default(),
            checksum_algorithm: Default::default(),
            tmpfilename: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The expected checksum of the file."]
    #[doc = ""]
    pub checksum: Option<String>,
    #[serde(rename = "checksum-algorithm")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The algorithm to calculate the checksum of the file."]
    #[doc = ""]
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    #[doc = "Content type."]
    #[doc = ""]
    pub content: Content,
    #[doc = "The name of the file to create. Caution: This will be normalized!"]
    #[doc = ""]
    pub filename: FilenameStr,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The source file name. This parameter is usually set by the REST handler. You can only overwrite it when connecting to the trusted port on localhost."]
    #[doc = ""]
    pub tmpfilename: Option<TmpfilenameStr>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The algorithm to calculate the checksum of the file."]
#[doc = ""]
pub enum ChecksumAlgorithm {
    #[serde(rename = "md5")]
    Md5,
    #[serde(rename = "sha1")]
    Sha1,
    #[serde(rename = "sha224")]
    Sha224,
    #[serde(rename = "sha256")]
    Sha256,
    #[serde(rename = "sha384")]
    Sha384,
    #[serde(rename = "sha512")]
    Sha512,
}
impl TryFrom<&str> for ChecksumAlgorithm {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "md5" => Ok(Self::Md5),
            "sha1" => Ok(Self::Sha1),
            "sha224" => Ok(Self::Sha224),
            "sha256" => Ok(Self::Sha256),
            "sha384" => Ok(Self::Sha384),
            "sha512" => Ok(Self::Sha512),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Content type."]
#[doc = ""]
pub enum Content {
    #[serde(rename = "iso")]
    Iso,
    #[serde(rename = "vztmpl")]
    Vztmpl,
}
impl TryFrom<&str> for Content {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "iso" => Ok(Self::Iso),
            "vztmpl" => Ok(Self::Vztmpl),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FilenameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for FilenameStr {
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
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for FilenameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TmpfilenameStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for TmpfilenameStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("/var/tmp/pveupload-[0-9a-f]+");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"/var/tmp/pveupload-[0-9a-f]+\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for TmpfilenameStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for TmpfilenameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for TmpfilenameStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
