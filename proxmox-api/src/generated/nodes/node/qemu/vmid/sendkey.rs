pub struct SendkeyClient<T> {
    client: T,
    path: String,
}
impl<T> SendkeyClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/sendkey"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a SendkeyClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> SendkeyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Send key event to virtual machine."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &params)
    }
}
impl PutParams {
    pub fn new(key: String) -> Self {
        Self {
            key,
            skiplock: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PutParams {
    #[doc = "The key (qemu monitor encoding)."]
    #[doc = ""]
    pub key: String,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Ignore locks - only root is allowed to use this option."]
    #[doc = ""]
    pub skiplock: Option<bool>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
