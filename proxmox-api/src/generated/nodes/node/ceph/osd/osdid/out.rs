#[derive(Debug, Clone)]
pub struct OutClient<T> {
    client: T,
    path: String,
}
impl<T> OutClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/out"),
        }
    }
}
impl<T> OutClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ceph osd out"]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/\", [\"Sys.Modify\"])"]
    pub async fn post(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &()).await
    }
}
