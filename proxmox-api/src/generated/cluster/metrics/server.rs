pub mod id;
#[derive(Debug, Clone)]
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
impl<T> ServerClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List configured metric servers."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(disable: bool, id: String, port: i64, server: String, ty: String) -> Self {
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
    pub port: i64,
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
