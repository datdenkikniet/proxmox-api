pub struct RestartClient<T> {
    client: T,
    path: String,
}
impl<T> RestartClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/restart"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a RestartClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> RestartClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Hard restart service. Use reload if you want to reduce interruptions."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
