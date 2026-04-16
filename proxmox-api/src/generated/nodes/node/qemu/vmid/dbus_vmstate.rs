#[derive(Debug, Clone)]
pub struct DbusVmstateClient<T> {
    client: T,
    path: String,
}
impl<T> DbusVmstateClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/dbus-vmstate"),
        }
    }
}
impl<T> DbusVmstateClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Control the dbus-vmstate helper for a given running VM."]
    #[doc = ""]
    pub fn post(&self, params: PostParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.post(&path, &params)
    }
}
impl PostParams {
    pub fn new(action: Action) -> Self {
        Self {
            action,
            additional_properties: ::std::default::Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct PostParams {
    #[doc = "Action to perform on the DBus VMState helper."]
    #[doc = ""]
    pub action: Action,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Action to perform on the DBus VMState helper."]
#[doc = ""]
pub enum Action {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "stop")]
    Stop,
}
impl TryFrom<&str> for Action {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "start" => Ok(Self::Start),
            "stop" => Ok(Self::Stop),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
