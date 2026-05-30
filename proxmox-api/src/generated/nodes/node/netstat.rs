#[derive(Debug, Clone)]
pub struct NetstatClient<T> {
    client: T,
    path: String,
}
impl<T> NetstatClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/netstat"),
        }
    }
}
impl<T> NetstatClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Read tap/vm network device interface counters"]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/nodes/{node}\", [\"Sys.Audit\"])"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutputItems {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
