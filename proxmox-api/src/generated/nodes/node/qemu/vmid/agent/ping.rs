pub struct PingClient<T> {
    client: T,
    path: String,
}
impl<T> PingClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/ping"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a PingClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> PingClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Execute ping."]
    #[doc = ""]
    pub fn post(&self) -> Result<PostOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.post(&path, &())
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
