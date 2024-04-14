pub mod name;
pub struct AccountClient<T> {
    client: T,
    path: String,
}
impl<T> AccountClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/account"),
        }
    }
}
impl<T> AccountClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ACMEAccount index."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> AccountClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Register a new ACME account with CA."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl PostParams {
    pub fn new(contact: String) -> Self {
        Self {
            contact,
            directory: Default::default(),
            eab_hmac_key: Default::default(),
            eab_kid: Default::default(),
            name: Default::default(),
            tos_url: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Contact email addresses."]
    #[doc = ""]
    pub contact: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL of ACME CA directory endpoint."]
    #[doc = ""]
    pub directory: Option<String>,
    #[serde(rename = "eab-hmac-key")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "HMAC key for External Account Binding."]
    #[doc = ""]
    pub eab_hmac_key: Option<String>,
    #[serde(rename = "eab-kid")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Key Identifier for External Account Binding."]
    #[doc = ""]
    pub eab_kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "ACME account config file name."]
    #[doc = ""]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "URL of CA TermsOfService - setting this indicates agreement."]
    #[doc = ""]
    pub tos_url: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> AccountClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
