pub struct ResumeClient<T> {
    client: T,
    path: String,
}
impl<T> ResumeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/resume"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ResumeClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ResumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Resume the container."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
