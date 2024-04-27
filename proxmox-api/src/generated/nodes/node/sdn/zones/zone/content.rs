#[derive(Debug, Clone)]
pub struct ContentClient<T> {
    client: T,
    path: String,
}
impl<T> ContentClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/content"),
        }
    }
}
impl<T> ContentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List zone content."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(vnet: String) -> Self {
        Self {
            vnet,
            status: Default::default(),
            statusmsg: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Status."]
    #[doc = ""]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Status details"]
    #[doc = ""]
    pub statusmsg: Option<String>,
    #[doc = "Vnet identifier."]
    #[doc = ""]
    pub vnet: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
