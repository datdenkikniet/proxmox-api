pub struct SuspendClient<T> {
    client: T,
    path: String,
}
impl<T> SuspendClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/suspend"),
        }
    }
}
impl<T> SuspendClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Suspend the container. This is experimental."]
    #[doc = ""]
    pub fn post(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &())
    }
}
