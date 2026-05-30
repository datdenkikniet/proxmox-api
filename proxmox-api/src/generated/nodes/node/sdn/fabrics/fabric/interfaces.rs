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
    #[doc = "Permission check: perm(\"/sdn/fabrics/{fabric}\", [\"SDN.Audit\"])"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
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
