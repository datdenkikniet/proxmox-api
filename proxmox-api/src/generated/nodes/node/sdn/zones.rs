pub mod zone;
#[derive(Debug, Clone)]
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
impl<T> ZonesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get status for all zones."]
    #[doc = ""]
    pub fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
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
    #[doc = ""]
    pub status: Status,
    #[doc = "The SDN zone object identifier."]
    #[doc = ""]
    pub zone: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Status of zone"]
#[doc = ""]
pub enum Status {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "pending")]
    Pending,
}
impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "available" => Ok(Self::Available),
            "error" => Ok(Self::Error),
            "pending" => Ok(Self::Pending),
            v => Err(format!("Unknown variant {v}")),
        }
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
