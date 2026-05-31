pub mod name;
#[derive(Debug, Clone)]
pub struct FsClient<T> {
    client: T,
    path: String,
}
impl<T> FsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/fs"),
        }
    }
}
impl<T> FsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Directory index."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/\", [\"Sys.Audit\", \"Datastore.Audit\"], any)"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(data_pool: String, metadata_pool: String, name: String) -> Self {
        Self {
            data_pool,
            metadata_pool,
            name,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The name of the data pool."]
    #[doc = ""]
    pub data_pool: String,
    #[doc = "The name of the metadata pool."]
    #[doc = ""]
    pub metadata_pool: String,
    #[doc = "The ceph filesystem name."]
    #[doc = ""]
    pub name: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> FsClient<T>
where
    T: crate::client::Client,
{
    pub fn name(&self, name: &str) -> name::NameClient<T> {
        name::NameClient::<T>::new(self.client.clone(), &self.path, name)
    }
}
