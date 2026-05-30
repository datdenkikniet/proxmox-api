#[derive(Debug, Clone)]
pub struct RoutesClient<T> {
    client: T,
    path: String,
}
impl<T> RoutesClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/routes"),
        }
    }
}
impl<T> RoutesClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get all routes for a fabric."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/sdn/fabrics/{fabric}\", [\"SDN.Audit\"])"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(route: String, via: Vec<String>) -> Self {
        Self {
            route,
            via,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[doc = "The CIDR block for this routing table entry."]
    #[doc = ""]
    pub route: String,
    #[serde(skip_serializing_if = "::std::vec::Vec::is_empty", default)]
    #[doc = "A list of nexthops for that route."]
    #[doc = ""]
    pub via: Vec<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
