#[derive(Debug, Clone)]
pub struct CpuClient<T> {
    client: T,
    path: String,
}
impl<T> CpuClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/cpu"),
        }
    }
}
impl<T> CpuClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List all custom and default CPU models."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(custom: bool, name: String, vendor: String) -> Self {
        Self {
            custom,
            name,
            vendor,
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
    #[doc = "True if this is a custom CPU model."]
    #[doc = ""]
    pub custom: bool,
    #[doc = "Name of the CPU model. Identifies it for subsequent API calls. Prefixed with 'custom-' for custom models."]
    #[doc = ""]
    pub name: String,
    #[doc = "CPU vendor visible to the guest when this model is selected. Vendor of 'reported-model' in case of custom models."]
    #[doc = ""]
    pub vendor: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
