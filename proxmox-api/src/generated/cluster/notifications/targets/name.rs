pub mod test;
pub struct NameClient<T> {
    client: T,
    path: String,
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, name: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, name),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a NameClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> NameClient<T>
where
    T: crate::client::Client,
{
    pub fn test(&self) -> test::TestClient<T> {
        test::TestClient::<T>::new(self.client.clone(), &self.path)
    }
}
