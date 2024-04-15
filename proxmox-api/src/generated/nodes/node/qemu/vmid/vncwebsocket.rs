pub struct VncwebsocketClient<T> {
    client: T,
    path: String,
}
impl<T> VncwebsocketClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/vncwebsocket"),
        }
    }
}
impl<'a, T> crate::ProxmoxClient for &'a VncwebsocketClient<T>
where
    T: crate::client::Client,
{
    type Path = &'a str;
    fn path(self) -> Self::Path {
        &self.path
    }
}
impl<T> VncwebsocketClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Opens a weksocket for VNC traffic."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = crate::ProxmoxClient::path(self).as_ref();
        self.client.get(&path, &params)
    }
}
impl GetOutput {
    pub fn new(port: String) -> Self {
        Self {
            port,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    pub port: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(port: u64, vncticket: String) -> Self {
        Self {
            port,
            vncticket,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(
        serialize_with = "crate::types::serialize_int",
        deserialize_with = "crate::types::deserialize_int"
    )]
    #[doc = "Port number returned by previous vncproxy call."]
    #[doc = ""]
    pub port: u64,
    #[doc = "Ticket from previous call to vncproxy."]
    #[doc = ""]
    pub vncticket: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
