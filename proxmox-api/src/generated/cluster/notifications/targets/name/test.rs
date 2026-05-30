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
    #[doc = "Permission check: or(perm(\"/mapping/notifications\", [\"Mapping.Modify\"]), perm(\"/mapping/notifications\", [\"Mapping.Audit\"]), perm(\"/mapping/notifications\", [\"Mapping.Use\"]))"]
    pub async fn post(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &()).await
    }
}
