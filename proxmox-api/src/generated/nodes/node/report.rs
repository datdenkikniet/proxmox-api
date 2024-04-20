pub struct ReportClient<T> {
    client: T,
    path: String,
}
impl<T> ReportClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/report"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a ReportClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> ReportClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Gather various systems information about a node"]
    #[doc = ""]
    pub fn get(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), String, T::Error> for &ReportClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<String, T::Error> {
        self.get()
    }
}
