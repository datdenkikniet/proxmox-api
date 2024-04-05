pub struct DumpClient<T> {
    client: T,
    path: String,
}
impl<T> DumpClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/dump"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Type {
    #[serde(rename = "meta")]
    Meta,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "user")]
    User,
}
impl GetParams {
    pub fn new(ty: Type) -> Self {
        Self {
            ty,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(rename = "type")]
    #[doc = "Config type."]
    pub ty: Type,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> DumpClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get automatically generated cloudinit config."]
    pub fn get(&self, params: GetParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
