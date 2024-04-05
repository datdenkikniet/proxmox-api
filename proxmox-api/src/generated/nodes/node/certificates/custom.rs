pub struct CustomClient<T> {
    client: T,
    path: String,
}
impl<T> CustomClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/custom"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct DeleteParams {
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Restart pveproxy."]
    pub restart: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> CustomClient<T>
where
    T: crate::client::Client,
{
    #[doc = "DELETE custom certificate chain and key."]
    pub fn delete(&self, params: DeleteParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &params)
    }
}
impl PostParams {
    pub fn new(certificates: String) -> Self {
        Self {
            certificates,
            force: Default::default(),
            key: Default::default(),
            restart: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "PEM encoded certificate (chain)."]
    pub certificates: String,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Overwrite existing custom or ACME certificate files."]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "PEM encoded private key."]
    pub key: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_bool_optional",
        deserialize_with = "crate::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Restart pveproxy."]
    pub restart: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate SHA 256 fingerprint."]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate issuer name."]
    pub issuer: Option<String>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's notAfter timestamp (UNIX epoch)."]
    pub notafter: Option<u64>,
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate's notBefore timestamp (UNIX epoch)."]
    pub notbefore: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Certificate in PEM format"]
    pub pem: Option<String>,
    #[serde(rename = "public-key-bits")]
    #[serde(
        serialize_with = "crate::serialize_int_optional",
        deserialize_with = "crate::deserialize_int_optional"
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
impl<T> CustomClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Upload or update custom certificate chain and key."]
    pub fn post(&self, params: PostParams) -> Result<PostOutput, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
