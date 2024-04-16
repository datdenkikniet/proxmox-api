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
impl<'a, T> crate::ProxmoxClient for &'a UploadClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> UploadClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Upload templates and ISO images."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, String, T::Error>
    for &UploadClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<String, T::Error> {
        self.post(params)
    }
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
    pub filename: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The source file name. This parameter is usually set by the REST handler. You can only overwrite it when connecting to the trusted port on localhost."]
    #[doc = ""]
    pub tmpfilename: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
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
