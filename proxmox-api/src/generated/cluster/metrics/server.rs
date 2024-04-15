pub mod id;
pub struct ServerClient<T> {
    client: T,
    path: String,
}
impl<T> ServerClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/server"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ServerClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ServerClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List configured metric servers."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(disable: bool, id: String, port: u64, server: String, ty: String) -> Self {
        Self {
            disable,
            id,
            port,
            server,
            ty,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::types::serialize_bool",
        deserialize_with = "crate::types::deserialize_bool"
    )]
    #[doc = "Flag to disable the plugin."]
    #[doc = ""]
    pub disable: bool,
    #[doc = "The ID of the entry."]
    #[doc = ""]
    pub id: String,
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Server network port"]
    #[doc = ""]
    pub port: u64,
    #[doc = "Server dns name or IP address"]
    #[doc = ""]
    pub server: String,
    #[serde(rename = "type")]
    #[doc = "Plugin type."]
    #[doc = ""]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ServerClient<T>
where
    T: crate::client::Client,
{
    pub fn id(&self, id: &str) -> id::IdClient<T> {
        id::IdClient::<T>::new(self.client.clone(), &self.path, id)
    }
}
