pub struct StartClient<T> {
    client: T,
    path: String,
}
impl<T> StartClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/start"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a StartClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> StartClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Start service."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
