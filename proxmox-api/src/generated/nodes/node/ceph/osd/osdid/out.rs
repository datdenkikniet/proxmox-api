#[derive(Debug, Clone)]
pub struct OutClient<T> {
    client: T,
    path: String,
}
impl<T> OutClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/out"),
        }
    }
}
impl<T> OutClient<T>
where
    T: crate::client::Client,
{
    #[doc = "ceph osd out"]
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
