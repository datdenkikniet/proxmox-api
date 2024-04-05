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
impl<T> RawClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get the Ceph configuration file."]
    pub fn get(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
