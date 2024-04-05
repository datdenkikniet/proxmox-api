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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Content {
    #[serde(rename = "iso")]
    Iso,
    #[serde(rename = "vztmpl")]
    Vztmpl,
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
    pub checksum: Option<String>,
    #[serde(rename = "checksum-algorithm")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The algorithm to calculate the checksum of the file."]
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Decompress the downloaded file using the specified compression algorithm."]
    pub compression: Option<String>,
    #[doc = "Content type."]
    pub content: Content,
    #[doc = "The name of the file to create. Caution: This will be normalized!"]
    pub filename: String,
    #[doc = "The URL to download the file from."]
    pub url: String,
    #[serde(rename = "verify-certificates")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "If false, no SSL/TLS certificates will be verified."]
    pub verify_certificates: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DownloadUrlClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Download templates and ISO images by using an URL."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
