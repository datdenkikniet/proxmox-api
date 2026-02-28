#[derive(Debug, Clone)]
pub struct VersionClient<T> {
    client: T,
    path: String,
}
impl<T> VersionClient<T>
where
    T: crate::client::Client,
{
    pub fn new(client: T) -> Self {
        Self {
            client,
            path: "/version".to_string(),
        }
    }
}
impl<T> VersionClient<T>
where
    T: crate::client::Client,
{
    #[doc = "API version details, including some parts of the global datacenter config."]
    #[doc = ""]
    pub fn get(&self) -> Result<GetOutput, T::Error> {
        let path = self.path.to_string();
        self.client.get(&path, &())
    }
}
impl GetOutput {
    pub fn new(release: String, repoid: RepoidStr, version: String) -> Self {
        Self {
            release,
            repoid,
            version,
            console: Default::default(),
            additional_properties: Default::default(),
        }
    }
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize)]
pub struct GetOutput {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    #[doc = "The default console viewer to use."]
    #[doc = ""]
    pub console: Option<Console>,
    #[doc = "The current Proxmox VE point release in `x.y` format."]
    #[doc = ""]
    pub release: String,
    #[doc = "The short git revision from which this version was build."]
    #[doc = ""]
    pub repoid: RepoidStr,
    #[doc = "The full pve-manager package version of this node."]
    #[doc = ""]
    pub version: String,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::collections::HashMap::is_empty"
    )]
    pub additional_properties: ::std::collections::HashMap<String, ::serde_json::Value>,
}
#[derive(Clone, Debug, :: serde :: Serialize, :: serde :: Deserialize, PartialEq)]
#[doc = "The default console viewer to use."]
#[doc = ""]
pub enum Console {
    #[serde(rename = "applet")]
    Applet,
    #[serde(rename = "html5")]
    Html5,
    #[serde(rename = "vv")]
    Vv,
    #[serde(rename = "xtermjs")]
    Xtermjs,
}
impl TryFrom<&str> for Console {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, <Self as TryFrom<&str>>::Error> {
        match value {
            "applet" => Ok(Self::Applet),
            "html5" => Ok(Self::Html5),
            "vv" => Ok(Self::Vv),
            "xtermjs" => Ok(Self::Xtermjs),
            v => Err(format!("Unknown variant {v}")),
        }
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RepoidStr {
    value: String,
}
impl crate::types::bounded_string::BoundedString for RepoidStr {
    const MIN_LENGTH: Option<usize> = None::<usize>;
    const MAX_LENGTH: Option<usize> = None::<usize>;
    const DEFAULT: Option<&'static str> = None::<&'static str>;
    const PATTERN: Option<&'static str> = Some("[0-9a-fA-F]{8,64}");
    const TYPE_DESCRIPTION: &'static str =
        "a string with pattern r\"[0-9a-fA-F]{8,64}\" and no length constraints";
    fn get_value(&self) -> &str {
        &self.value
    }
    fn new(value: String) -> Result<Self, crate::types::bounded_string::BoundedStringError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }
}
impl std::convert::TryFrom<String> for RepoidStr {
    type Error = crate::types::bounded_string::BoundedStringError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        crate::types::bounded_string::BoundedString::new(value)
    }
}
impl ::serde::Serialize for RepoidStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        crate::types::serialize_bounded_string(self, serializer)
    }
}
impl<'de> ::serde::Deserialize<'de> for RepoidStr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        crate::types::deserialize_bounded_string(deserializer)
    }
}
