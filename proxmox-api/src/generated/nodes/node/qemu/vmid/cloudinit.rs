pub mod dump;
pub struct CloudinitClient<T> {
    client: T,
    path: String,
}
impl<T> CloudinitClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/cloudinit"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a CloudinitClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> CloudinitClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the cloudinit configuration with both current and pending values."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> CloudinitClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Regenerate and change cloudinit config drive."]
    #[doc = ""]
    pub fn put(&self) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.put(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(key: String) -> Self {
        Self {
            key,
            delete: Default::default(),
            pending: Default::default(),
            value: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_int_optional",
        deserialize_with = "crate::types::deserialize_int_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Indicates a pending delete request if present and not 0."]
    #[doc = ""]
    pub delete: Option<u64>,
    #[doc = "Configuration option name."]
    #[doc = ""]
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The new pending value."]
    #[doc = ""]
    pub pending: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Value as it was used to generate the current cloudinit image."]
    #[doc = ""]
    pub value: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> CloudinitClient<T>
where
    T: crate::client::Client,
{
    pub fn dump(&self) -> dump::DumpClient<T> {
        dump::DumpClient::<T>::new(self.client.clone(), &self.path)
    }
}
