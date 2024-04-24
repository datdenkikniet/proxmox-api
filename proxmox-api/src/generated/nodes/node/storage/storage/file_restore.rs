pub mod download;
pub mod list;
#[derive(Debug, Clone)]
pub struct FileRestoreClient<T> {
    client: T,
    path: String,
}
impl<T> FileRestoreClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/file-restore"),
        }
    }
}
impl<T> FileRestoreClient<T>
where
    T: crate::client::Client,
{
    pub fn list(&self) -> list::ListClient<T> {
        list::ListClient::<T>::new(self.client.clone(), &self.path)
    }
}
impl<T> FileRestoreClient<T>
where
    T: crate::client::Client,
{
    pub fn download(&self) -> download::DownloadClient<T> {
        download::DownloadClient::<T>::new(self.client.clone(), &self.path)
    }
}
