pub struct CertificateClient<T> {
    client: T,
    path: String,
}
impl<T> CertificateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/certificate"),
        }
    }
}
impl<T> CertificateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Revoke existing certificate from CA."]
    #[doc = ""]
    pub fn delete(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> CertificateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Order a new certificate from ACME-compatible CA."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl<T> CertificateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Renew existing certificate from CA."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Overwrite existing custom certificate."]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Force renewal even if expiry is more than 30 days away."]
    #[doc = ""]
    pub force: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
