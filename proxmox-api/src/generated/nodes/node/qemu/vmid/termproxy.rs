pub struct TermproxyClient<T> {
    client: T,
    path: String,
}
impl<T> TermproxyClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/termproxy"),
        }
    }
}
impl<T> TermproxyClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Creates a TCP proxy connections."]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PostParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "opens a serial terminal (defaults to display)"]
    pub serial: Option<Serial>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub enum Serial {
    #[serde(rename = "serial0")]
    Serial0,
    #[serde(rename = "serial1")]
    Serial1,
    #[serde(rename = "serial2")]
    Serial2,
    #[serde(rename = "serial3")]
    Serial3,
}
