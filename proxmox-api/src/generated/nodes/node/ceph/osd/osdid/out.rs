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
impl<'a, T> crate::ProxmoxClient for &'a OutClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> OutClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ceph osd out"]
    #[doc = ""]
    pub fn post(&self) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), (), T::Error> for &OutClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: ()) -> Result<(), T::Error> {
        self.post()
    }
}
