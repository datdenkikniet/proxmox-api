#[derive(Debug, Clone)]
pub struct ZfsClient<T> {
    client: T,
    path: String,
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/zfs"),
        }
    }
}
impl<T> ZfsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Scan zfs pool list on local node."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/storage\", [\"Datastore.Allocate\"])"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(pool: String) -> Self {
        Self {
            pool,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "ZFS pool name."]
    #[doc = ""]
    pub pool: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
