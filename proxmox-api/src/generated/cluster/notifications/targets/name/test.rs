pub struct TestClient<T> {
    client: T,
    path: String,
}
impl<T> TestClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/test"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a TestClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> TestClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Send a test notification to a provided target."]
    #[doc = ""]
    pub fn post(&self) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), (), T::Error> for &TestClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: ()) -> Result<(), T::Error> {
        self.post()
    }
}
