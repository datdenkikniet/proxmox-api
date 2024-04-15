pub struct MetaClient<T> {
    client: T,
    path: String,
}
impl<T> MetaClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/meta"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a MetaClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> MetaClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Retrieve ACME Directory Meta Information"]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(rename = "caaIdentities")]
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "Hostnames referring to the ACME servers."]
    #[doc = ""]
    pub caaidentities: Vec<String>,
    #[serde(rename = "externalAccountRequired")]
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "EAB Required"]
    #[doc = ""]
    pub externalaccountrequired: Option<bool>,
    #[serde(rename = "termsOfService")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ACME TermsOfService URL."]
    #[doc = ""]
    pub termsofservice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL to more information about the ACME server."]
    #[doc = ""]
    pub website: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL of ACME CA directory endpoint."]
    #[doc = ""]
    pub directory: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
