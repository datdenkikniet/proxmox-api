pub struct ApiversionClient<T> {
    client: T,
    path: String,
}
impl<T> ApiversionClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/apiversion"),
        }
    }
}
impl<T> ApiversionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Return the version of the cluster join API available on this node."]
    pub fn get(&self) -> Result<u64, T::Error> {
        let path = self.path.to_string();
        Ok(self
            .client
            .get::<_, crate::types::Integer>(&path, &())?
            .get())
    }
}
