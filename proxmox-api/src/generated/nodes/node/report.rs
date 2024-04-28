#[derive(Debug, Clone)]
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
impl<T> ReportClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Gather various systems information about a node"]
    #[doc = ""]
    pub fn get(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
