pub struct StartClient<T> {
    client: T,
    path: String,
}
impl<T> StartClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/start"),
        }
    }
}
impl<T> StartClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Start service."]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &())
    }
}
