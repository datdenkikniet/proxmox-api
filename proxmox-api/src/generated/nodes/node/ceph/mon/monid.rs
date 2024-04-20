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
impl<'a, T> crate::ProxmoxClient for &'a MonidClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> MonidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Destroy Ceph Monitor and Manager."]
    #[doc = ""]
    pub fn delete(&self) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.delete(&path, &())
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<(), String, T::Error> for &MonidClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Delete;
    fn exec(&self, params: ()) -> Result<String, T::Error> {
        self.delete()
    }
}
impl<T> MonidClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Create Ceph Monitor and Manager"]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<String, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &params)
    }
}
impl<T> crate::proxmox_client::ProxmoxClientAction<PostParams, String, T::Error> for &MonidClient<T>
where
    T: crate::client::Client,
{
    const METHOD: crate::client::Method = crate::client::Method::Post;
    fn exec(&self, params: PostParams) -> Result<String, T::Error> {
        self.post(params)
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
