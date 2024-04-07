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
    #[doc = "Get IP addresses of the specified container interface."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(hwaddr: String, name: String) -> Self {
        Self {
            hwaddr,
            name,
            inet: Default::default(),
            inet6: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The MAC address of the interface"]
    pub hwaddr: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The IPv4 address of the interface"]
    pub inet: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The IPv6 address of the interface"]
    pub inet6: Option<String>,
    #[doc = "The name of the interface"]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
