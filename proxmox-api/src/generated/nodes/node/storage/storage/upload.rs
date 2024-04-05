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
    pub fn new(content: Content, filename: String) -> Self {
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
    pub checksum: Option<String>,
    #[serde(rename = "checksum-algorithm")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The algorithm to calculate the checksum of the file."]
    pub checksum_algorithm: Option<ChecksumAlgorithm>,
    #[doc = "Content type."]
    pub content: Content,
    #[doc = "The name of the file to create. Caution: This will be normalized!"]
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The source file name. This parameter is usually set by the REST handler. You can only overwrite it when connecting to the trusted port on localhost."]
    pub tmpfilename: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> UploadClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Upload templates and ISO images."]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
