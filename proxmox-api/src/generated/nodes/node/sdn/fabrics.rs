pub mod fabric;
#[derive(Debug, Clone)]
pub struct FabricsClient<T> {
    client: T,
    path: String,
}
impl<T> FabricsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/fabrics"),
        }
    }
}
impl<T> FabricsClient<T>
where
    T: crate::client::Client,
{
    pub fn fabric(&self, fabric: &str) -> fabric::FabricClient<T> {
        fabric::FabricClient::<T>::new(self.client.clone(), &self.path, fabric)
    }
}
