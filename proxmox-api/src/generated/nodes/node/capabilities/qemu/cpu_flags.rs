#[derive(Debug, Clone)]
pub struct CpuFlagsClient<T> {
    client: T,
    path: String,
}
impl<T> CpuFlagsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/cpu-flags"),
        }
    }
}
impl<T> CpuFlagsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List of available VM-specific CPU flags."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(description: String, name: String) -> Self {
        Self {
            description,
            name,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Description of the CPU flag."]
    #[doc = ""]
    pub description: String,
    #[doc = "Name of the CPU flag."]
    #[doc = ""]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
