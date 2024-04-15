pub struct InNameClient<T> {
    client: T,
    path: String,
}
impl<T> InNameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/in"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a InNameClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> InNameClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ceph osd in"]
    #[doc = ""]
    pub fn post(&self) -> Result<(), T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
