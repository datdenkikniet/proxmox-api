#[derive(Debug, Clone)]
pub struct MonidClient<T> {
    client: T,
    path: String,
}
impl<T> MonidClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str, monid: &str) -> Self {
        Self {
            client,
            path: format!("{}/{}", parent_path, monid),
        }
    }
}
impl<T> MonidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Destroy Ceph Monitor and Manager."]
    #[doc = ""]
    pub fn delete(&self) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.delete(&path, &())
    }
}
impl<T> MonidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create Ceph Monitor and Manager"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(rename = "mon-address")]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Overwrites autodetected monitor IP address(es). Must be in the public network(s) of Ceph."]
    #[doc = ""]
    pub mon_address: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
