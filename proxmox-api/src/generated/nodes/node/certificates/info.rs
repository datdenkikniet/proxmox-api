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
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate issuer name."]
    pub issuer: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's notAfter timestamp (UNIX epoch)."]
    pub notafter: Option<u64>,
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's notBefore timestamp (UNIX epoch)."]
    pub notbefore: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate in PEM format"]
    pub pem: Option<String>,
    #[serde(rename = "public-key-bits")]
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's public key size"]
    pub public_key_bits: Option<u64>,
    #[serde(rename = "public-key-type")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's public key algorithm"]
    pub public_key_type: Option<String>,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "List of Certificate's SubjectAlternativeName entries."]
    pub san: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate subject name."]
    pub subject: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> InfoClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get information about node's certificates."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
