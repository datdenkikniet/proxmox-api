#[derive(Debug, Clone)]
pub struct WakeonlanClient<T> {
    client: T,
    path: String,
}
impl<T> WakeonlanClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/wakeonlan"),
        }
    }
}
impl<T> WakeonlanClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Try to wake a node via 'wake on LAN' network packet."]
    #[doc = ""]
    pub fn post(&self) -> Result<crate::types::MacAddr<true>, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &())
    }
}
