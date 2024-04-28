#[derive(Debug, Clone)]
pub struct CrushClient<T> {
    client: T,
    path: String,
}
impl<T> CrushClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/crush"),
        }
    }
}
impl<T> CrushClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get OSD crush map"]
    #[doc = ""]
    pub fn get(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
