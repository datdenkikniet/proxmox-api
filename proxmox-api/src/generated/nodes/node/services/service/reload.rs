#[derive(Debug, Clone)]
pub struct ReloadClient<T> {
    client: T,
    path: String,
}
impl<T> ReloadClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/reload"),
        }
    }
}
impl<T> ReloadClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Reload service. Falls back to restart if service cannot be reloaded."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &())
    }
}
