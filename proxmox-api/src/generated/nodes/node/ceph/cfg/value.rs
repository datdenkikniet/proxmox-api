#[derive(Debug, Clone)]
pub struct ValueClient<T> {
    client: T,
    path: String,
}
impl<T> ValueClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T, parent_path: &str) -> Self {
        Self {
            client,
            path: format!("{}{}", parent_path, "/value"),
        }
    }
}
impl<T> ValueClient<T>
where
    T: crate::client::Client,
{
    #[doc = "Get configured values from either the config file or config DB."]
    #[doc = ""]
    pub fn get(&self, params: GetParams) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &params)
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, Default)]
pub struct GetOutput {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
impl GetParams {
    pub fn new(config_keys: ConfigKeysStr) -> Self {
        Self {
            config_keys,
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetParams {
    #[serde(rename = "config-keys")]
    #[doc = "List of \\\\<section\\\\>:\\\\<config key\\\\> items."]
    #[doc = ""]
    pub config_keys: ConfigKeysStr,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConfigKeysStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for ConfigKeysStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some(
        "(?^:^(:?(?^i:[0-9a-z\\-_\\.]+:[0-9a-zA-Z\\-_]+))(:?[;, ](?^i:[0-9a-z\\-_\\.]+:[0-9a-zA-Z\\-_]+))*$)",
    );
    const TYPE_DESCRIPTION: &'static str = "a string with pattern r\"(?^:^(:?(?^i:[0-9a-z\\-_\\.]+:[0-9a-zA-Z\\-_]+))(:?[;, ](?^i:[0-9a-z\\-_\\.]+:[0-9a-zA-Z\\-_]+))*$)\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for ConfigKeysStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for ConfigKeysStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for ConfigKeysStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
