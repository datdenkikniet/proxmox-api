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
impl<'a, T> crate::ProxmoxClient for &'a ReloadClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ReloadClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Reload service. Falls back to restart if service cannot be reloaded."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
