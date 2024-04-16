pub struct SuspendClient<T> {
    client: T,
    path: String,
}
impl<T> SuspendClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/suspend"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a SuspendClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> SuspendClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Suspend the container. This is experimental."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), String, T::Error> for &SuspendClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: ()) -> Result<String, T::Error> {
        self.post()
    }
}
