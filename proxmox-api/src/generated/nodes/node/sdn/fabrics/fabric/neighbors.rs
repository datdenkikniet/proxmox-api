#[derive(Debug, Clone)]
pub struct NeighborsClient<T> {
    client: T,
    path: String,
}
impl<T> NeighborsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/neighbors"),
        }
    }
}
impl<T> NeighborsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get all neighbors for a fabric."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/sdn/fabrics/{fabric}\", [\"SDN.Audit\"])"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(neighbor: String, status: String, uptime: String) -> Self {
        Self {
            neighbor,
            status,
            uptime,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The IP or hostname of the neighbor."]
    #[doc = ""]
    pub neighbor: String,
    #[doc = "The status of the neighbor, as returned by FRR."]
    #[doc = ""]
    pub status: String,
    #[doc = "The uptime of this neighbor, as returned by FRR (e.g. 8h24m12s)."]
    #[doc = ""]
    pub uptime: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
