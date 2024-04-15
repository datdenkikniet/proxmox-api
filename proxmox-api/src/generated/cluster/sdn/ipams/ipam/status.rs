pub struct StatusClient<T> {
    client: T,
    path: String,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/status"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a StatusClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List PVE IPAM Entries"]
    #[doc = ""]
    pub fn get(&self) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
