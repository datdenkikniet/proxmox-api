#[derive(Debug, Clone)]
pub struct InterfacesClient<T> {
    client: T,
    path: String,
}
impl<T> InterfacesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/interfaces"),
        }
    }
}
impl<T> InterfacesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get all interfaces for a fabric."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(name: String, state: String, ty: String) -> Self {
        Self {
            name,
            state,
            ty,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The name of the network interface."]
    #[doc = ""]
    pub name: String,
    #[doc = "The current state of the interface."]
    #[doc = ""]
    pub state: String,
    #[serde(rename = "type")]
    #[doc = "The type of this interface in the fabric (e.g. Point-to-Point, Broadcast, ..)."]
    #[doc = ""]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
