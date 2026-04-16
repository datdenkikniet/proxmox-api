pub mod vnet;
#[derive(Debug, Clone)]
pub struct VnetsClient<T> {
    client: T,
    path: String,
}
impl<T> VnetsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/vnets"),
        }
    }
}
impl<T> VnetsClient<T>
where
    T: crate::client::Client,
{
    pub fn vnet(&self, vnet: &str) -> vnet::VnetClient<T> {
        vnet::VnetClient::<T>::new(self.client.clone(), &self.path, vnet)
    }
}
