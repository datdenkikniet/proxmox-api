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
impl<'a, T> crate::ProxmoxClient for &'a IdClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Destroy Ceph Manager."]
    #[doc = ""]
    pub fn delete(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.delete(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), String, T::Error> for &IdClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Delete;
    fn exec(&self, params: ()) -> Result<String, T::Error> {
        self.delete()
    }
}
impl<T> IdClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create Ceph Manager"]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), String, T::Error> for &IdClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: ()) -> Result<String, T::Error> {
        self.post()
    }
}
