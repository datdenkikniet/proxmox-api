pub mod not_backed_up;
#[derive(Debug, Clone)]
pub struct BackupInfoClient<T> {
    client: T,
    path: String,
}
impl<T> BackupInfoClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/backup-info"),
        }
    }
}
impl<T> BackupInfoClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index for backup info related endpoints"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutputItems {
    pub fn new(subdir: String) -> Self {
        Self {
            subdir,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "API sub-directory endpoint"]
    #[doc = ""]
    pub subdir: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> BackupInfoClient<T>
where
    T: crate::client::Client,
{
    pub fn not_backed_up(&self) -> not_backed_up::NotBackedUpClient<T> {
        not_backed_up::NotBackedUpClient::<T>::new(self.client.clone(), &self.path)
    }
}
