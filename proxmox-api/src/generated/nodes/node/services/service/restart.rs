pub struct RestartClient<T> {
    client: T,
    path: String,
}
impl<T> RestartClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/restart"),
        }
    }
}
impl<T> RestartClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Hard restart service. Use reload if you want to reduce interruptions."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &())
    }
}
