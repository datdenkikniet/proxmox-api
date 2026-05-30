#[derive(Debug, Clone)]
pub struct IdClient<T> {
    client: T,
    path: String,
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, id: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, id),
        }
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Destroy Ceph Manager."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/\", [\"Sys.Modify\"])"]
    pub async fn delete(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &()).await
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create Ceph Manager"]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/\", [\"Sys.Modify\"])"]
    pub async fn post(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &()).await
    }
}
