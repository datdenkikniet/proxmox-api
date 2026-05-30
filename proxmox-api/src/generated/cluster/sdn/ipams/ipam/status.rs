#[derive(Debug, Clone)]
pub struct StatusClient<T> {
    client: T,
    path: String,
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/status"),
        }
    }
}
impl<T> StatusClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List PVE IPAM Entries"]
    #[doc = ""]
    #[doc = "Only list entries where you have 'SDN.Audit' or 'SDN.Allocate' permissions on '/sdn/zones/\\<zone\\>/\\<vnet\\>'"]
    pub async fn get(&self) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &()).await
    }
}
