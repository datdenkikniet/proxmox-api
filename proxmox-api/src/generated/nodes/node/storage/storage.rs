pub mod content;
pub mod download_url;
pub mod file_restore;
pub mod import_metadata;
pub mod prunebackups;
pub mod rrd;
pub mod rrddata;
pub mod status;
pub mod upload;
#[derive(Debug, Clone)]
pub struct StorageClient<T> {
    client: T,
    path: String,
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, storage: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, storage),
        }
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
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
    pub subdir: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn prunebackups(&self) -> prunebackups::PrunebackupsClient<T> {
        prunebackups::PrunebackupsClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn content(&self) -> content::ContentClient<T> {
        content::ContentClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn file_restore(&self) -> file_restore::FileRestoreClient<T> {
        file_restore::FileRestoreClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn status(&self) -> status::StatusClient<T> {
        status::StatusClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn rrd(&self) -> rrd::RrdClient<T> {
        rrd::RrdClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn rrddata(&self) -> rrddata::RrddataClient<T> {
        rrddata::RrddataClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn upload(&self) -> upload::UploadClient<T> {
        upload::UploadClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn download_url(&self) -> download_url::DownloadUrlClient<T> {
        download_url::DownloadUrlClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> StorageClient<T>
where
    T: crate::client::Client,
{
    pub fn import_metadata(&self) -> import_metadata::ImportMetadataClient<T> {
        import_metadata::ImportMetadataClient::<T>::new(self.client.clone(), &self.path)
    }
}
