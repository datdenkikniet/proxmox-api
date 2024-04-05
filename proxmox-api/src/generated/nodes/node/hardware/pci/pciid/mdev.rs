pub struct MdevClient<T> {
    client: T,
    path: String,
}
impl<T> MdevClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/mdev"),
        }
    }
}
impl GetOutputItems {
    pub fn new(available: u64, description: String, ty: String) -> Self {
        Self {
            available,
            description,
            ty,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(
        serialize_with = "crate::serialize_int",
        deserialize_with = "crate::deserialize_int"
    )]
    #[doc = "The number of still available instances of this type."]
    pub available: u64,
    pub description: String,
    #[serde(rename = "type")]
    #[doc = "The name of the mdev type."]
    pub ty: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MdevClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List mediated device types for given PCI device."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
