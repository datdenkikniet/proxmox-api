pub struct RawClient<T> {
    client: T,
    path: String,
}
impl<T> RawClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/raw"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a RawClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> RawClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the Ceph configuration file."]
    #[doc = ""]
    pub fn get(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), String, T::Error> for &RawClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<String, T::Error> {
        self.get()
    }
}
