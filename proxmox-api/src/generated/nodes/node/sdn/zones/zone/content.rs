#[derive(Debug, Clone)]
pub struct ContentClient<T> {
    client: T,
    path: String,
}
impl<T> ContentClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/content"),
        }
    }
}
impl<T> ContentClient<T>
where
    T: crate::client::Client,
{
    #[doc = "List zone content."]
    #[doc = ""]
    #[doc = "Permission check: perm(\"/sdn/zones/{zone}\", [\"SDN.Audit\"])"]
    pub async fn get(&self) -> Result<Vec<GetOutputItems>, T::Error> {
        let path = self.path.to_string();
        let optional_vec: Option<Vec<GetOutputItems>> = self.client.get(&path, &()).await?;
        Ok(optional_vec.unwrap_or_default())
    }
}
impl GetOutputItems {
    pub fn new(vnet: String) -> Self {
        Self {
            vnet,
            status: ::std::default::Default::default(),
            statusmsg: ::std::default::Default::default(),
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutputItems {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Status."]
    #[doc = ""]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Status details"]
    #[doc = ""]
    pub statusmsg: Option<String>,
    #[doc = "Vnet identifier."]
    #[doc = ""]
    pub vnet: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
