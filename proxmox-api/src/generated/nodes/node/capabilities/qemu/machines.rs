pub struct MachinesClient<T> {
    client: T,
    path: String,
}
impl<T> MachinesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/machines"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "i440fx")]
    I440fx,
    #[serde(rename = "q35")]
    Q35,
}
impl GetOutputItems {
    pub fn new(id: String, ty: Type, version: String) -> Self {
        Self {
            id,
            ty,
            version,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Full name of machine type and version."]
    pub id: String,
    #[serde(rename = "type")]
    #[doc = "The machine type."]
    pub ty: Type,
    #[doc = "The machine version."]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MachinesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get available QEMU/KVM machine types."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
