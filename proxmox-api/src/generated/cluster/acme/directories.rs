pub struct DirectoriesClient<T> {
    client: T,
    path: String,
}
impl<T> DirectoriesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/directories"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a DirectoriesClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> DirectoriesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get named known ACME directory endpoints."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), Vec<GetOutputItems>, T::Error>
    for &DirectoriesClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Get;
    fn exec(&self, params: ()) -> Result<Vec<GetOutputItems>, T::Error> {
        self.get()
    }
}
impl GetOutputItems {
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    pub name: String,
    #[doc = "URL of ACME CA directory endpoint."]
    #[doc = ""]
    pub url: String,
}
