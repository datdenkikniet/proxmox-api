pub struct MtunnelwebsocketClient<T> {
    client: T,
    path: String,
}
impl<T> MtunnelwebsocketClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/mtunnelwebsocket"),
        }
    }
}
impl GetParams {
    pub fn new(socket: String, ticket: String) -> Self {
        Self {
            socket,
            ticket,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[doc = "unix socket to forward to"]
    pub socket: String,
    #[doc = "ticket return by initial 'mtunnel' API call, or retrieved via 'ticket' tunnel command"]
    pub ticket: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub socket: Option<String>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl<T> MtunnelwebsocketClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Migration tunnel endpoint for websocket upgrade - only for internal use by VM migration."]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
