#[derive(Debug, Clone)]
pub struct InfoClient<T> {
    client: T,
    path: String,
}
impl<T> InfoClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/info"),
        }
    }
}
impl<T> InfoClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get information about node's certificates."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    #[doc = ""]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate issuer name."]
    #[doc = ""]
    pub issuer: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's notAfter timestamp (UNIX epoch)."]
    #[doc = ""]
    pub notafter: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's notBefore timestamp (UNIX epoch)."]
    #[doc = ""]
    pub notbefore: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate in PEM format"]
    #[doc = ""]
    pub pem: Option<String>,
    #[serde(rename = "public-key-bits")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's public key size"]
    #[doc = ""]
    pub public_key_bits: Option<u64>,
    #[serde(rename = "public-key-type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's public key algorithm"]
    #[doc = ""]
    pub public_key_type: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of Certificate's SubjectAlternativeName entries."]
    #[doc = ""]
    pub san: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate subject name."]
    #[doc = ""]
    pub subject: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
