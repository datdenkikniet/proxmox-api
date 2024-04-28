#[derive(Debug, Clone)]
pub struct TestClient<T> {
    client: T,
    path: String,
}
impl<T> TestClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/test"),
        }
    }
}
impl<T> TestClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Send a test notification to a provided target."]
    #[doc = ""]
    pub fn post(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        match self.client.post(&path, &()) {
            Ok(o) => Ok(o),
            Err(e) if crate::client::Error::is_empty_data(&e) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
