#[derive(Debug, Clone)]
pub struct OptionsClient<T> {
    client: T,
    path: String,
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/options"),
        }
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get vnet firewall options."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl<T> OptionsClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Set Firewall options."]
    #[doc = ""]
    pub fn put(&self, params: PutParams) -> Result<(), T::Error> {
        let path = self.path.to_string();
        self.client.put(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable firewall rules."]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for forwarded traffic."]
    #[doc = ""]
    pub log_level_forward: Option<LogLevelForward>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Forward policy."]
    #[doc = ""]
    pub policy_forward: Option<PolicyForward>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct PutParams {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "A list of settings you want to delete."]
    #[doc = ""]
    pub delete: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Prevent changes if current configuration file has a different digest. This can be used to prevent concurrent modifications."]
    #[doc = ""]
    pub digest: Option<String>,
    #[serde(
        serialize_with = "crate::types::serialize_bool_optional",
        deserialize_with = "crate::types::deserialize_bool_optional"
    )]
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Enable/disable firewall rules."]
    #[doc = ""]
    pub enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Log level for forwarded traffic."]
    #[doc = ""]
    pub log_level_forward: Option<LogLevelForward>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "Forward policy."]
    #[doc = ""]
    pub policy_forward: Option<PolicyForward>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Log level for forwarded traffic."]
#[doc = ""]
pub enum LogLevelForward {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "crit")]
    Crit,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "emerg")]
    Emerg,
    #[serde(rename = "err")]
    Err,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "nolog")]
    Nolog,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
}
impl TryFrom<&str> for LogLevelForward {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "alert" => Ok(Self::Alert),
            "crit" => Ok(Self::Crit),
            "debug" => Ok(Self::Debug),
            "emerg" => Ok(Self::Emerg),
            "err" => Ok(Self::Err),
            "info" => Ok(Self::Info),
            "nolog" => Ok(Self::Nolog),
            "notice" => Ok(Self::Notice),
            "warning" => Ok(Self::Warning),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "Forward policy."]
#[doc = ""]
pub enum PolicyForward {
    ACCEPT,
    DROP,
}
impl TryFrom<&str> for PolicyForward {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "ACCEPT" => Ok(Self::ACCEPT),
            "DROP" => Ok(Self::DROP),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
