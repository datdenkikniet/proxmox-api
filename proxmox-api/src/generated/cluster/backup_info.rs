pub mod not_backed_up;
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
impl<'a, T> crate::ProxmoxClient for &'a BackupInfoClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> BackupInfoClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Index for backup info related endpoints"]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &BackupInfoClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
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
