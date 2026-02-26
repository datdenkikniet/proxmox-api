#[derive(Debug, Clone)]
pub struct DownloadUrlClient<T> {
    client: T,
    path: String,
}
impl<T> DownloadUrlClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/download-url"),
        }
    }
}
impl<T> DownloadUrlClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Download templates, ISO images, OVAs and VM images by using an URL."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(content: Content, filename: String, url: String) -> Self {
        Self {
            content,
            filename,
            url,
            checksum: Default::default(),
            checksum_algorithm: Default::default(),
            compression: Default::default(),
            verify_certificates: Default::default(),
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Decompress the downloaded file using the specified compression algorithm."]
    #[doc = ""]
    pub compression: Option<String>,
    #[doc = "Content type."]
    #[doc = ""]
    pub content: Content,
    #[doc = "The name of the file to create. Caution: This will be normalized!"]
    #[doc = ""]
    pub filename: String,
    #[doc = "The URL to download the file from."]
    #[doc = ""]
    pub url: String,
    #[serde(rename = "verify-certificates")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If false, no SSL/TLS certificates will be verified."]
    #[doc = ""]
    pub verify_certificates: Option<bool>,
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
    #[serde(rename = "import")]
    Import,
    #[serde(rename = "iso")]
    Iso,
    #[serde(rename = "vztmpl")]
    Vztmpl,
}
impl TryFrom<&str> for Content {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "import" => Ok(Self::Import),
            "iso" => Ok(Self::Iso),
            "vztmpl" => Ok(Self::Vztmpl),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
