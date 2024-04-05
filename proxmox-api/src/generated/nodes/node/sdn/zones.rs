pub mod zone;
pub struct ZonesClient<T> {
    client: T,
    path: String,
}
impl<T> ZonesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/zones"),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Status {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "pending")]
    Pending,
}
impl GetOutputItems {
    pub fn new(status: Status, zone: String) -> Self {
        Self {
            status,
            zone,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "Status of zone"]
    pub status: Status,
    #[doc = "The SDN zone object identifier."]
    pub zone: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> ZonesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get status for all zones."]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> ZonesClient<T>
where
    T: crate::client::Client,
{
    pub fn zone(&self, zone: &str) -> zone::ZoneClient<T> {
        zone::ZoneClient::<T>::new(self.client.clone(), &self.path, zone)
    }
}
