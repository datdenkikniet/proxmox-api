#[derive(Debug, Clone)]
pub struct TemplateClient<T> {
    client: T,
    path: String,
}
impl<T> TemplateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/template"),
        }
    }
}
impl<T> TemplateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create a Template."]
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
