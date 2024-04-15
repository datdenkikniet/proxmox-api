pub struct ApiversionClient<T> {
    client: T,
    path: String,
}
impl<T> ApiversionClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/apiversion"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ApiversionClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ApiversionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Return the version of the cluster join API available on this node."]
    #[doc = ""]
    pub fn get(&self) -> Result<u64, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        Ok(self
            .client
            .get::<_, crate::types::Integer>(&path, &())?
            .get())
    }
}
