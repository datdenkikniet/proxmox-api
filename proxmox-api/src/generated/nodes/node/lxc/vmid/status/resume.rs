pub struct ResumeClient<T> {
    client: T,
    path: String,
}
impl<T> ResumeClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/resume"),
        }
    }
}
impl<T> ResumeClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Resume the container."]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &())
    }
}
