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
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List PVE IPAM Entries"]
    #[doc = ""]
    pub fn get(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
